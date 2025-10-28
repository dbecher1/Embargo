# embargo-cpp

A C++ build tool for the modern world.

## Motivation

Inspired by Rust's Cargo in more than just name, the vision for Embargo is to create a build system that brings accessibility to the novice programmer, while not sacrificing power, performance and customizability for the power user.

This tool is just barely in its infancy, so almost if not all features mentioned are likely to be in progress, if not entirely yet unimplemented. The purpose for this README is, as of now, mostly to cast vision for how exciting Embargo can and should be.

## Project Status

The project is in a minimally working alpha phase currently. Currently working functionality will be outlined below; the next steps are as follows:

- Properly implement default platform paths
- Implement Github actions - build tests for cross-platform compilation, unit tests for functionality, and pre-compiled binaries
- Implement external package management (add/remove command)
- Finish implementing clean functionality
- Bug test more thoroughly on target platforms; additionally, clean up exception handling
- Add more flags for currently implemented commands
- Add color to output (low priority)

## Requirements

 - A C++ compiler, defaulting to `g++` (and currently only supporting g++)
 - `git` for version control

## Installation

If `cargo` is installed, the source code can be compiled as easily as running `cargo build --release` from within the project directory. I intend to have pre-compiled binaries available for download within the near future. The binary can then be added to the PATH environment variable, however the user chooses to do so.

## Usage

The following commands are implemented; in lieu of further documentation, information on command flags can be obtained by adding the -h flag. 

- `embargo init`
    - Initializes a new C++ project in the current working directory
- `embargo new <NAME>`
    - Within the current working directory, creates a new directory NAME, and a C++ project of the same name within the directory.
- `embargo build`
    - Compiles an Embargo project, as long as there is an Embargo.toml file within the CWD or a parent directory (rather, as long as the CWD is within an embargo project)
- `embargo run`
    - Builds and runs an embargo project - will run `build` before running
- `embargo clean`
    - Removes build artifacts from the build folder - WIP, may not fully work