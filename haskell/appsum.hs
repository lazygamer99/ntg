ape :: Int->[Int]->[Int]
ape x []=[x]
ape x (y:ys)=y : ape x ys

sumlist :: [Int]->Int
sumlist []=0
sumlist (x:xs)=x + sumlist(xs)

main :: IO()
main = do
    print $ ape 4 [1,2,3]     -- Prints the 4th element of the list [1,2,3]
    print $ sumlist [1,2,3,4] -- Prints the sum of the list [1,2,3,4]
