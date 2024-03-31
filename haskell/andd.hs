andd :: Bool -> Bool -> Bool
andd True True = True
andd a b = False

orp :: Bool -> Bool -> Bool
orp False False = False
orp a b = True

xorp :: Bool -> Bool -> Bool
xorp False True = True
xorp True False = True
xorp b1 b2 = False

main :: IO()
main = print $ andd True False
