sumtuple :: [(Int, Int)] -> Int
sumtuple [] = 0
sumtuple ((x, y):zs) = x + y + sumtuple zs

main :: IO ()
main = do
    let tuples = [(5, 4), (1, 2)]
    putStrLn "Input Tuples:"
    mapM_ (\tuple -> putStrLn $ "Tuple: " ++ show tuple ++ ", Sum: " ++ show (sumtuple [tuple])) tuples

