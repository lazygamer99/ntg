import Data.Char
capti :: Char->Char
capti ch
   | ('A'<=ch && ch<='Z')=chr(ord ch -(ord 'A' - ord 'a'))
   | otherwise = ch
main :: IO()
main = print $ capti 'A'
