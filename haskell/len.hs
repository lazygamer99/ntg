listlength :: [Int]->Integer
listlength []=0
listlength (x:xs)=1+listlength(xs)

revList :: [Int] -> [Int]
revList [] = []
revList (x:xs) = (revList xs) ++ [x]

main :: IO ()
main = do
    let myList = [1, 2, 3, 4, 5]  -- Define myList using let
        rl = reverse myList       -- Define rl using let
    print $ listlength myList            -- Print the length of myList
    print rl                      -- Print the reversed list
    print $ revList [1,2,3,4,5]
