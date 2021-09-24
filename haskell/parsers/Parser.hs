module Parser where 

import Control.Monad.State.Lazy
import Control.Applicative
import Data.Char

type Parser a = StateT String [] a
-- instance of monad plus because [] is a monadplus
-- which is also an instance of alternative

parser = StateT 
parse = runStateT

-- atoms
item :: Parser Char
item = parser $ (\s -> case s of [] -> []
                                 c:cs -> [(c, cs)])

satisfies :: (Char -> Bool) -> Parser Char
satisfies p = do c <- item
                 if p c then return c else empty

-- primitives
char :: Char -> Parser Char
char c = satisfies (c==)

digit :: Parser Char
digit = satisfies isDigit

alphaNum :: Parser Char
alphaNum = satisfies isAlphaNum

letter :: Parser Char
letter = satisfies isAlpha

identifierChar :: Parser Char
identifierChar = alphaNum <|> char '_'

number :: Parser Int
number = do s <- some digit
            return (read s)

string :: String -> Parser String
string []     = return ""
string (c:cs) = do head <- char c
                   tail <- string cs
                   return $ head:tail

identifier :: Parser String
identifier = do c <- letter <|> char '_'
                cs <- many identifierChar
                return (c:cs)

space :: Parser ()
space = do many (satisfies isSpace)
           return ()

token :: Parser a -> Parser a
token parser = do space
                  v <- parser
                  space
                  return v

identifierT :: Parser String 
identifierT = token identifier 

keyword :: String -> Parser String
keyword word = token (string (map toUpper word) <|> string (map toLower word))

someKeyword :: [String] -> Parser String
someKeyword [] = mzero
someKeyword (word:words) =  keyword word <|> someKeyword words

charT :: Char -> Parser Char
charT c = token (char c)
