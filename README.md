# Word-Unscrambler
This python program outputs a list of the possible options a scrambled word could be.

## About 
This program was created by [Martin Chaperot-Merino](https://github.com/tinmarr)

# How to use
1. Go [here](http://word-unscrambler.tinmarr.repl.run/)
2. Load the Website <br />
![](step1.png)
3. Enter a scrambled word <br />
![](step2.png)
4. Hit enter <br />
![](step3.png)

# How it works
It takes words from a text file and uses a lookup function to find words with the same letters (were the order of words does not matter).

## The key to its speed
It converts all the words into integers (which is based on the letters) and groups words with the same integer in a dictionary. Then it converts the typed word into an integer and looks up that integer in the dictionary.
