# collatz_2d

## About

This code is meant to try out my extension on the Collatz iterative formula.

## Prerequisites

This software does not have any dependencies.

## How to Compile

After downloading the code, install the Rust compiler: `curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh`.
This command works on Linux and MacOS. For more detailed instructions click [here](https://doc.rust-lang.org/book/ch01-01-installation.html).
If you want to check whether cargo is in your PATH run `which cargo`. In case it is not sourced yet, run `source ~/.cargo/env`. After
establishing that cargo is in your PATH go to the project directory and run

```bash
cargo build --release
```

This will procude a project_home/target/release directory, in which you can find the collatz_2d executable.

## The Original Math Formula

The original itarative formula, that Lothar Collatz invented in 1937, has the following form; take a positive whole number *n*<sub>0</sub>.
If *n*<sub>0</sub> is even, take half of it, otherwise calculate 3*n*<sub>0</sub>+1. This new number is *n*<sub>1</sub>. Itarate this procedure
infinitely many times (...*n*<sub>2</sub>, *n*<sub>3</sub>, *n*<sub>4</sub> ...). It is conjectured, that eventually every such starting 
number will get in the cycle of 1 -> 4 -> 2 -> 1 i.e., for every *n*<sub>0</sub> > 0 there exists an index *m*, such that *n*<sub>*m*</sub> = 1.

## My Extension

Picture this iterative formula as a game, the goal of which is to reach 1 (i.e., to reach the aformentioned cycle). The players of this game are the
positive whole numbers and the rules are the following: we want each "player" to have the desirable property of evenness. We "reward" this property
by halving the number and bringing it closer to winning. However, if the number is odd, we want to
1. "guide" this number back to the "right path" through making it even, but
2. also "punish" this number by making it larger.

That's why we apply 3x+1 to odd numbers; it makes them even, but it also grows them. Using these terms, the original question is reformulated to 
"which numbers win this game", for which the conjecture states that "all of them".

I wanted to create a similar iterative formula for **posivie whole number pairs**, that also produces interesting behaviour, applying the same
"logic", that governs the original equation in some sense.

Following this type of thinking I made the following system; start with a positive whole number pair (*x*<sub>0</sub>, *y*<sub>0</sub>). The desirable
subset in our case is when both *x*<sub>0</sub> and *y*<sub>0</sub> are even, or both are odd. In this case we can apply the original formula to both
of our numbers. However, if one is even and one is odd, we should get back to our desirable subset, but by "punishing" the number pair. My solution 
to this is by addig 1 to the bigger number. Summerized:
- If *x*<sub>0</sub> is even and *y*<sub>0</sub> is even, then *x*<sub>1</sub> = *x*<sub>0</sub>/2 and *y*<sub>1</sub> = *y*<sub>0</sub>/2
- If *x*<sub>0</sub> is odd and *y*<sub>0</sub> is odd, then *x*<sub>1</sub> = 3*x*<sub>0</sub>+1 and *y*<sub>1</sub> = 3*y*<sub>0</sub>+1
- If *x*<sub>0</sub> is even and *y*<sub>0</sub> is odd OR *x*<sub>0</sub> is odd and *y*<sub>0</sub> is even, then 
  - if *x*<sub>0</sub> > *y*<sub>0</sub>, then *x*<sub>1</sub> = *x*<sub>0</sub>+1 and *y*<sub>1</sub> = *y*<sub>0</sub>
  - if *x*<sub>0</sub> < *y*<sub>0</sub>, then *x*<sub>1</sub> = *x*<sub>0</sub> and *y*<sub>1</sub> = *y*<sub>0</sub>+1
  
By adding 1 to the bigger number we get further away from smaller numbers, but we assure that we get back to the desirable subset of number-pairs.

## Results

If you run the binary built from the original source code, you'll run every (*x*<sub>0</sub>, *y*<sub>0</sub>) pair, where both variables start between
0 and 1999. This can be modified by the RECTANGLE_SIZE constant. To avoid really long iterations a MAX_ITER constant is also provided, that enforces
an upper bound on every number-pair chain length. However, for a RECTANGLE_SIZE of 2000 every number pair falls into a cycle (attractor), so a 
MAX_ITER > 10000 is a bit of an overkill.

In this rectange 6 different cycles are present:
1. A trivial cycle of (0, 0) -> ...
2. another trivial cycle of (0, 1) -> (0, 2) -> ...
3. the original Collatz-cycle of (1, 1) -> (4, 4) -> (2, 2) -> ...
4. a cycle of (1, 3) -> (4, 10) -> (2, 5) -> (2, 6) -> ...
5. a cycle of (1, 4) -> (1, 5) -> (4, 16) -> (2, 8) -> ...
6. a cycle of (1, 6) -> (1, 7) -> (4, 22) -> (2, 11) -> (2, 12) -> ...

These cycles are populated in the following way: 1, 2000, 4358, 2998418, 566418, and 945963 (these are populations counted on the upper 
triangle of the reactange!).

Also, the sofware produces a "results.txt" output file containing the upper triangular part of the rectangle. This shows that to which cycle is the given
(*x*<sub>0</sub>, *y*<sub>0</sub>) index pair attracted to. Looking at this, we can see that the upper left corner (0, 0) is attracted to cycle 0,
the top row (0, *n*) is attracted to cycle 2, and the main diagonal (*n*, *n*) is attracted to cycle 3. The remaining pairs are devided into regions,
based on to which cycles (4, 5, or 6) the region's pairs are attracted to. Seemingly, these regions are separated by linear boundaries.
