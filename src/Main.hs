module Main where

main :: IO ()
main = do
  let 
    sum = fib 5
    msg = "welcome to frieght"
          ++ "\n"
          ++ show sum
   in putStrLn msg

fib :: Int -> Int
fib 0 = 0
fib 1 = 1
fib x = fib (x-2) + fib (x-1)
