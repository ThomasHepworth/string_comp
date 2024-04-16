### Acknowledgements

Information taken from both the [Informatica Documentation](https://docs.informatica.com/data-catalog/common-content-for-data-catalog/10-5/transformation-language-reference/functions/metaphone.html#:~:text=METAPHONE%20encodes%20characters%20of%20the,character%20of%20the%20input%20string.) and the [Wikipedia page](https://en.wikipedia.org/wiki/Metaphone) for Metaphone.

<hr>

# The Algorithm's Rules and Steps

## Constant Rules

1. **Drop all vowels unless it is the beginning**
2. **Drop double consonants except for C**: CC -> CC, DD -> D, EE -> E etc.
3. **'0' is a stand-in character representing the 'th' sound in English**: 'th' -> 0.
4. **'X' represents 'sh' or 'ch'**: 'sh' -> X, 'ch' -> X.

## Letter based rules table for Metaphone

```
| Input | Returns | Condition                                              |
|-------|---------|--------------------------------------------------------|
| B     | n/a     | when it follows M                                      |
| B     | B       | in all other cases                                     |
| C     | X       | when followed by IA or H                               |
| C     | S       | when followed by I, E, or Y                            |
| C     | n/a     | when it follows S, and is followed by I, E, or Y       |
| C     | K       | in all other cases                                     |
| D     | J       | when followed by GE, GY, or GI                         |
| D     | T       | in all other cases                                     |
| F     | F       | in all cases                                           |
| G     | F       | when followed by H and the first character is not B, D, or H |
| G     | n/a     | when followed by H and the first character is B, D, or H |
| G     | J       | when followed by I, E or Y and does not repeat         |
| G     | K       | in all other cases                                     |
| H     | H       | when it does not follow C, G, P, S, or T and is followed by A, E, I, or U |
| H     | n/a     | in all other cases                                     |
| J     | J       | in all cases                                           |
| K     | n/a     | when it follows C                                      |
| K     | K       | in all other cases                                     |
| L     | L       | in all cases                                           |
| M     | M       | in all cases                                           |
| N     | N       | in all cases                                           |
| P     | F       | when followed by H                                     |
| P     | P       | in all other cases                                     |
| Q     | K       | in all cases                                           |
| R     | R       | in all cases                                           |
| S     | X       | when followed by H, IO, IA, or CHW                     |
| S     | S       | in all other cases                                     |
| T     | X       | when followed by IA or IO                              |
| T     | 0       | when followed by H                                     |
| T     | n/a     | when followed by CH                                    |
| T     | T       | in all other cases                                     |
| V     | F       | in all cases                                           |
| W     | W       | when followed by A, E, I, O, or U                      |
| W     | n/a     | in all other cases                                     |
| X     | KS      | in all cases                                           |
| Y     | Y       | when followed by A, E, I, O, or U                      |
| Y     | n/a     | in all other cases                                     |
| Z     | S       | in all cases                                           |
```

<br>

## Examples

<details>
<summary>Click to expand/collapse the table of phonetic conversions</summary>

```markdown
| Input               | Conversion |
| ------------------- | ---------- |
| METAPHONE('Lamb')   | LM         |
| METAPHONE('Box')    | BKS        |
| METAPHONE('Facial') | FXL        |
| METAPHONE('Fence')  | FNS        |
| METAPHONE('Scene')  | SN         |
| METAPHONE('Cool')   | KL         |
| METAPHONE('Dodge')  | TJ         |
| METAPHONE('David')  | TFT        |
| METAPHONE('FOX')    | FKS        |
| METAPHONE('Tough')  | TF         |
| METAPHONE('Hugh')   | HF         |
| METAPHONE('Magic')  | MJK        |
| METAPHONE('GUN')    | KN         |
| METAPHONE('DHAT')   | THT        |
| METAPHONE('Chain')  | XN         |
| METAPHONE('Jen')    | JN         |
| METAPHONE('Ckim')   | KM         |
| METAPHONE('Kim')    | KM         |
| METAPHONE('Laura')  | LR         |
| METAPHONE('Maggi')  | MK         |
| METAPHONE('Nancy')  | NNS        |
| METAPHONE('Phone')  | FN         |
| METAPHONE('Pip')    | PP         |
| METAPHONE('Queen')  | KN         |
| METAPHONE('Ray')    | R          |
| METAPHONE('Cash')   | KX         |
| METAPHONE('Sing')   | SNK        |
| METAPHONE('Patio')  | PX         |
| METAPHONE('Thor')   | 0R         |
| METAPHONE('Glitch') | KLTX       |
| METAPHONE('Tim')    | TM         |
| METAPHONE('Vin')    | FN         |
| METAPHONE('Wang')   | WNK        |
| METAPHONE('When')   | HN         |
| METAPHONE('Six')    | SKS        |
| METAPHONE('Yang')   | YNK        |
| METAPHONE('Bobby')  | BB         |
| METAPHONE('Zack')   | SK         |
```

</details>

<br>

## Further information

For further information on how metaphone works (alongside other phonetic transformations), please refer to the [Splink documentation](https://moj-analytical-services.github.io/splink/topic_guides/comparisons/phonetic.html?h=metaphone#metaphone).
