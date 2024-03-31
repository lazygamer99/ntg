isEven :: Integral a => a -> Bool
isEven x = x `mod` 2 == 0

filterEven :: Integral a => [a] -> [a]
filterEven = map (\x -> if isEven x then x else 0)

removeDuplicates :: Eq a => [a] -> [a]
removeDuplicates [] = []
removeDuplicates (x:xs) = x : removeDuplicates (filter (/= x) xs)

main :: IO ()
main = do
   let inputList = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
       evenElements = filterEven inputList
       uniqueList = removeDuplicates evenElements
   putStrLn $ "Original list: " ++ show inputList
   putStrLn $ "Even elements: " ++ show uniqueList
       

