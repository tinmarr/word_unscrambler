# Word-Unscrambler
This python program outputs a list of the possible options a scrambled word could be.

## About 
This program was created by [Martin Chaperot-Merino](https://github.com/tinmarr)

# How to use
1. Open the IDE: [http://word-unscrambler.tinmarr.repl.run/](http://word-unscrambler.tinmarr.repl.run/)
2. Wait for the Prompt <br />
![The code asks to enter a scrambled word](step1.png)
3. Enter a scrambled word <br />
![The entered word is noctitusiont](step2.png)
4. Hit enter <br />
![The code return hello and asks if you want to restart](step3.png)

# How it works
It takes words from a text file and uses a lookup function to find words with the same letters (were the order of words does not matter).

## The key to its speed
It converts all the words into integers (which is based on the letters) and groups words with the same integer in a dictionary. Then it converts the typed word into an integer and looks up that integer in the dictionary.
