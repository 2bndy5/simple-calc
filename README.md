# simple-calc

A simple calculator created using the bevy game engine.

This was forked from its original source on github:
<https://github.com/PravinKumar95/simple-calc>
Modifications were made to:

- upgrade it to use bevy v0.18.1
- some code cleanup
- implement better expression parser/evaluator
- re-organized the UI based on Microsoft calculator ("standard" variant)

![image of simple calculator](/calc.PNG "bevy calc")

## Try it out

Checkout the wasm build hosted on this repo's GitHub Pages site:
<https://2bndy5.github.io/simple-calc/>

Or build it from source code:

1. Clone this repo:

   ```sh
   git clone https://github.com/2bndy5/simple-calc
   ```

2. Navigate to the locally cloned repo:

   ```sh
   cd simple-calc
   ```

3. Run the project binary (main.rs):

   ```sh
   cargo run
   ```
