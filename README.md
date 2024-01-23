#MiniGrep

## Description
This is a small rust program that acts as a smaller-scale version of grep (it does not contain every feature grep does). 
To use it, your arguments should be a regular expression and a local file to scan. If matching text in your file is found it will be shown in your terminal along with what line the text was at, the matching text highlighted red, and the rest of the line the match was found in. The output of your query can be piped into a new file to save the results, however the text will not be highlighted. 
