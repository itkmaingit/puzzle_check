# Overview

This is a research library for the theme of "Automatic Creation of Puzzle Rules". It is used to corroborate that the complete boards created by the puzzle rules defined in this study are sufficient conditions for the complete boards of existing puzzle rules. This library is released under the MIT License, so please proceed with development in accordance with its terms.

## Requirement

- Git
- Linux (WSL)
- Rust (cargo)

## Directory Structure

```
.
├── Cargo.lock
├── Cargo.toml
├── LICENSE
├── readme.md
├── run.sh                                --> You can output the complete board by executing sh run.sh [puzzle_name].
├── rustfmt.toml
└── src
    ├── bin
    │   └──[puzzle_name].rs               --> Programs corresponding to each puzzle rule.
    ├── common
    │   ├── combine.rs                    --> Contains programs for composition operations.
    │   ├── dataclass.rs                  --> Defines the structures used in this research.
    │   ├── initialize.rs                 --> Initializes sequences of elements.
    │   ├── operate_structures.rs         --> Contains programs for various operations on structures.
    │   ├── relationship.rs               --> Defines adjacency relationships.
    │   └── mod.rs
    ├── lib.rs
    └── specific
        ├── board_validation.rs           --> Describes predicates that the board structures must satisfy.
        ├── cutoff.rs                     --> Contains functions to eliminate structures that cannot exist in the puzzle rule.
        ├── predicates.rs                 --> Describes various predicates used for the puzzle rules.
        ├── structure_functions.rs        --> Contains programs to calculate the parameters that the structures possess.
        └── mod.rs

```

## How to run

```
git clone https://github.com/itkmaingit/puzzle_check
cd puzzle_check
```

The rs files in the `src/bin` folder correspond to each puzzle rule. To corroborate them, execute the following command:

```
sh run.sh [puzzle_name]
```

Executing this will create a file named `data/[puzzle_name].txt`, and the complete board will be outputted. Please verify that these match the complete boards of existing puzzle rules. As the visualization tool is still under development, you will need to check manually.
