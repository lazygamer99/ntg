and' :: Bool -> Bool -> Bool
and' True True = True
and' _ _ = False

-- Function for logical OR
or' :: Bool -> Bool -> Bool
or' False False = False
or' _ _ = True

-- Function for logical XOR
xor' :: Bool -> Bool -> Bool
xor' True False = True
xor' False True = True
xor' _ _ = False

main :: IO ()
main = do
    putStrLn "Enter the first boolean value (True/False):"
    input1 <- readLn :: IO Bool
    putStrLn "Enter the second boolean value (True/False):"
    input2 <- readLn :: IO Bool
    
    putStrLn "Logical AND:"
    print (and' input1 input2)
    
    putStrLn "Logical OR:"
    print (or' input1 input2)
    
    putStrLn "Logical XOR:"
    print (xor' input1 input2)

