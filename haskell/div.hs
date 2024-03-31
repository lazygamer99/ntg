divide :: Double -> Double -> Maybe Double
divide _ 0 = Nothing
divide x y = Just (x / y)

main :: IO ()
main = do
   let numerator = 10
       denominator = 0
       result = divide numerator denominator
   case result of
     Just res -> putStrLn $ "Result of division: " ++ show res
     Nothing -> putStrLn "Error: Division by zero"
