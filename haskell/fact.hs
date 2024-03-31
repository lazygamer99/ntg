fact :: Integer -> Integer
fact n
   | n == 0 = 1
   | otherwise = n * fact (n-1)
   
main :: IO ()
main = do
   putStrLn "Enter an integer: "
   number <- readLn :: IO Integer 
   putStrLn $ "Factorial of " ++ show number ++ " is " ++ show (fact number)


