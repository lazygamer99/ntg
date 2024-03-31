fac :: Int -> [Int]
fac n = [x | x <- [1..n], n `mod` x == 0]
prime :: Int -> Bool
prime n = fac n == [1, n]
primeli :: Int -> [Int]
primeli n = [x | x <- [2..n], prime x == True]
main :: IO ()
main = print $ primeli 25
