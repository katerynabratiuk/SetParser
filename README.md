# Set Parser
 Set Parser lets you declare, compare and do operations on sets. Useful for first-year CS students who only start Discrete Math to get a rough understanding of Set Theory.
 In this language, a set consists of integers that are separated by comma (not neccessarily true in real Set Theory!).
 See [grammar](./parser/src/grammar.pest) for more details.
 You can:

### 1. Declare and define a set
```
let A = {1, 2, 4};
let C = {1..10};
let D = ∅; // this is an empty set (yes, comments are available too)
```


### 2. Perform operations on sets
```
let E = A \ C;
let F = A ∩ C;
```

or something more complicated:
```
let E = (A' \ C) △ B;
let F = (A ∩ B)' △ (A △ B)
```

Available operations are:
- A' - complement (universe set must be declared)
- A ∪ B - union
- A ∩ B - intersection
- A \ B - difference
- A △ B - symmetric difference

### 3. Set universe value (useful for complements)
```
let universe = {1, 2, 3, 4, 5}
```

Example of usage:
```
let universe = {1, 2, 3, 4, 5}
let A = {1, 2}
let B = A'
print B
```
will output {3, 4, 5}  

### 4. Print a set
```
print A
print A △ B
```
