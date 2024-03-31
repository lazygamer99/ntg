decimalToInt :: [Int] -> Int
decimalToInt digits = foldl (\acc digit -> acc * 10 + digit) 0 digits

main :: IO ()
main = do
    let decimalDigits = [1, 2, 3, 4, 5]  -- Define decimalDigits using let
        result = decimalToInt decimalDigits  -- Define result using let
    print result  -- Print the result
    putStrLn $ "Decimal: " ++ show decimalDigits
    putStrLn $ "Integer: " ++ show result
