# Json Parser

This project represent the solution for the second coding challenge: https://codingchallenges.fyi/challenges/challenge-json-parser

## Usage

## Reference level explanation

The json parser will be split into 2 part, the Lexer and the Static Analyser. We will explore both components in this chapter.

## Syntactic analyzer

In order to implement the syntactic analyzer we are going to implement the Recursive Descent Parser(RDP) algorithm. 
To implement the RDP we first need a grammar as RDP is a topdown algorithm that recursively parses an input according to the grammar rules.

Our grammar will be defined in BNF (Backus Naur Form) as below:

```
<json> ::= <object> | <array>
<object> ::= "{" <members>? "}"
<members> ::= <pair> ("," <pair>)*
<pair> ::= <string> ":" <value>
<array> ::= "[" <elements>? "]"
<elements> ::= <value> ("," <value>)*
<value> ::= <string> | <number> | <object> | <array> | "true" | "false" | "null"
<string> ::= '"' <char>* '"'
<char> ::= <unicode> | "\\" <escape>
<unicode> ::= any Unicode character except '"' and '\'
<escape> ::= '"' | "\" | "/" | "b" | "f" | "n" | "r" | "t" | "u" <hex> <hex> <hex> <hex>
<number> ::= <int> | <int> "." <frac> | <int> <exp> | <int> "." <frac> <exp>
<int> ::= <digit> | <digit> <int>
<frac> ::= <digit>* <digit>
<exp> ::= ("e" | "E") ("+" | "-")? <digit>*
<digit> ::= "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9"
<hex> ::= <digit> | "A" | "B" | "C" | "D" | "E" | "F" | "a" | "b" | "c" | "d" | "e" | "f"
```

The grammar above will not be implemented entirely for this example but the code demonstrates the parsing of basic json excluding complex corner-cases.

Please NOTE: the current implementation of the syntactic analyser needs to be reworked because it is not able to support inner objects and arrays.

## Conclusion

This json parser is a pure didactic project, unable to handle json edge cases. It is not maintained and will not be continued.