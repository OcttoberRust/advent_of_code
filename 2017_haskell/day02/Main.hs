 
 -- Main.hs
 import System.IO
 import System.Environment (getArgs)

 main :: IO ()
 main = do
     args <- getArgs
     input <- case args of
                   [fileName] -> readFile fileName
                   _          -> getContents
                   let result = solve input
                       print result

                       solve :: String -> Int
                       solve input = length (lines input)
