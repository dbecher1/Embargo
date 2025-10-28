# embargo-cpp

A C++ build tool for the modern world.

[![Crates.io Version](https://img.shields.io/crates/v/embargo-cpp)](https://crates.io/crates/embargo-cpp)
[![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/dbecher1/embargo-cpp/rust.yml)](https://github.com/dbecher1/embargo-cpp/actions)

## Motivation

Inspired by Rust's Cargo in more than just name, the vision for Embargo is to create a build system that brings accessibility to the novice programmer, while not sacrificing power, performance and customizability for the power user.

This tool is just barely in its infancy, so almost if not all features mentioned are likely to be in progress, if not entirely yet unimplemented. The purpose for this README is, as of now, mostly to cast vision for how exciting Embargo can and should be.

## Project Status and Roadmap

The project is in a minimally working alpha phase currently. Init, New, Build and Run commands are implemented with limited flags/options. For planned features, see the Github issues for this repository.

## Requirements

 - A C++ compiler, defaulting to `g++` (and currently only supporting g++)
 - `git` for version control

## Installation

`cargo install embargo-cpp`

This will install the embargo binary.

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