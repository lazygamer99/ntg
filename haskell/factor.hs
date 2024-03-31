fac :: Int->[Int]
fac n=[x|x<-[1..n],n`mod`x==0]
main :: IO()
main = print $ fac 21
