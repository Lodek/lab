import Parser
import Control.Applicative
import Data.Char

data Operator
    = Equality
    | NEquality
    | LIKE
    | IN
    | IS
    | LT
    | GT
    | GET
    | LET
    deriving (Show, Eq)

data Literal
    = Str String
    | Num Int
    | Boolean Bool
    | List [Literal]
    | Null
    deriving (Show, Eq)

data Combinator
    = And
    | Or
    deriving (Show, Eq)

data Operand
    = Column String
    | Value Literal
    deriving (Eq, Show)

data Predicate = Predicate Operand Operator Operand deriving(Show, Eq)

data Query = Node Query Combinator Query | Leaf Predicate deriving (Show, Eq)

strUpper = map toUpper

columnIdentifier :: Parser String
columnIdentifier = identifierT <|> do
    char '`'
    id <- identifierT
    char '`'
    return id

str :: Parser Literal
str = token $ do 
    char '\''
    value <- many $ satisfies ('\''/=) --does not work with escaped quotes
    char '\''
    return (Str value)

int :: Parser Literal
int = token $ do
    n <- number
    return (Num n)

bool :: Parser Literal
bool = do {keyword "true"; return (Boolean True)} <|> do {keyword "false"; return (Boolean False)}

nullT :: Parser Literal
nullT = do keyword "null"
           return Null

literal :: Parser Literal
literal = str <|> int <|> bool <|> nullT

list :: Parser [Literal]
list = do charT '['
          values <- some $ do {s <- str; charT ','; return s}
          charT ']'
          return values

          
operand :: Parser Operand
operand = do {l <- literal; return (Value l)} <|> do {id <- columnIdentifier; return (Column id)}

operatorMap :: String -> Operator
operatorMap op
    | strUpper op == "LIKE" = LIKE
    | strUpper op == "IN" = IN
    | strUpper op == "IS" = IS
    | op == "="  = Equality
    | op == "<>" = NEquality

operator :: Parser Operator
operator = do keyword <- someKeyword ["like", "in", "<>", "is", "="]
              return (operatorMap keyword)

predicate :: Parser Predicate
predicate = do 
    a <- operand
    op <- operator
    b <- operand
    return (Predicate a op b)

combinatorMap :: String -> Combinator
combinatorMap token
    | strUpper token == "AND" = And
    | strUpper token == "OR" = Or

combinator :: Parser Combinator
combinator = do keyword <- someKeyword ["and", "or"]
                return (combinatorMap keyword)

query :: Parser Query
query = do p1 <- predicate
           p1' <- return (Leaf p1)
           rest <- many $ do {comb <- combinator; p2 <- predicate; return (comb, p2)}
           return (foldl (\tree (comb, p2) -> Node tree comb (Leaf p2)) p1' rest)
