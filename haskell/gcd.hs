mygcd :: Integer -> Integer -> Integer
mygcd a 0 = a
mygcd a b
  | a>=b = gcd b (mod a b)
  | otherwise = gcd b a
main :: IO()
main = do
   putStrLn "Enter an integer: "
   number1 <- readLn :: IO Integer
   putStrLn "Enter an integer: "
   number2 <- readLn :: IO Integer 
   putStrLn $ "GDC of " ++ show number1 ++ "&" ++show number2 ++ " is " ++ show (mygcd number1 number2)
