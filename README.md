# 3d-cellular-automata
An open-source 3D [cellular automata](https://en.wikipedia.org/wiki/Cellular_automaton) created in Rust for my Honours Project at university.
I am hoping to port this to Web Assembly (WASM) to be hosted on [my website](https://lewismorris.dev).

# What language?
- Rust

## Why Rust?
- Fast and optimised
  - Up to twenty-six calculations per cell, per frame...**a lot**

### Learning Goals
- GUIs
  - Familiarise myself with creating a GUI
- Learn/ Apply good UI/UX design concepts and practices
  - Applying practices such as KISS (Keep-It-Simple-and-Stupid)
- Familiarise myself with the Rust language
- Manage a self-directed project

# Aims and objectives

## Core
- Implement as a single-threaded application
- Tests using pre-calculated starting states
- Create a platform which is able to host different types of cells
  - Model of competition
  - These "competing" cells would not be counted as valid neighbours
- Implement both [Moore](https://en.wikipedia.org/wiki/Moore_neighborhood) and [Von Neumann](https://en.wikipedia.org/wiki/Von_Neumann_neighborhood) neighbourhood algorithms

## Desirable
- Command line menu to allow user to change settings
- Creation of certain presets
- Implement as a multithreaded application
- Some basic commands
  - e.g. a help command with an explanation of each setting

## Aspirational
- Port to WASM
- Implement a settings GUI
  - Should allow user to:
    - Change bounding box size
    - Toggle neighbourhood algorithm
    - Change cycles before cell death
    - Change number of cells to sustain
    - Change various other settings/ rules
- Addition of some special rules
  - Custom neighbourhood algorithm
- Change the colour palette
- _**Run parallel calculations on GPU**_
  - This is a **long-term** goal 

# Packages used
- _See Cargo.toml for a list of dependencies_
- **Bevy** >= 0.9.1
- **bevy_egui** = "0.17.1"
  - Not currently in use
- **bytemuck** = "1.12.3"
  - Dependency for bevy_egui