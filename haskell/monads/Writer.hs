newtype Writer w a = Writer { runWriter :: (a, w) } deriving (Show)

instance Monoid w => Monad (Writer w) where
     -- >>= :: Writer a -> (a -> Writer b) -> Writer b
     (Writer (a, w)) >>= f = let (b, w') = runWriter $ f a in Writer (b, (w `mappend` w'))
     return a = Writer (a, mempty)

instance Functor (Writer w) where
    fmap f (Writer (a, w)) = Writer (f a, w)

instance Monoid w => Applicative (Writer w) where
    -- <*> :: Writer w (a -> b) -> Writer w a -> Writer w b
    Writer (f, w) <*> Writer (a, w') = Writer (f a, w `mappend` w')
    pure = return

sumM x y = Writer (x + y, ["Added"])

multM x y = Writer (x * y, ["Mult"])

minusM x y = Writer (x - y, ["Sub"])

divM x y = Writer (x / y, ["Div"])

summingCalc = do 
    r <- sumM 1 3
    r' <-divM r 1
    multM r' 0
