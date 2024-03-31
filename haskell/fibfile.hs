fibo :: Integer -> Integer
fibo 0 = 1
fibo 1 = 1
fibo n
   | n < 0 = fibo (-n)
   | n > 0 = fibo (n-1) + fibo (n-2)
main :: IO()
main = do
   putStrLn "Enter an integer: "
   number <- readLn :: IO Integer 
   putStrLn $ "Factorial of " ++ show number ++ " is " ++ show (fibo number)

