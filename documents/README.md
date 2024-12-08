# Embargo Notes

This is a working document that I'm keeping just as a public notes kind of deal. This is mainly for my brain to keep things in order and what have you - but given the infancy of the project, I see no harm in keeping a public record of these notes.

## Commands

### Init 
Creates a new project in the CWD with the name of the current directory. Initializes git repository of the same name, clones appropirate files into project.

### New
Creates a new directory in the CWD with the name of the argument, from there same behavior as init.

### Build
Some requirements:
- Support compiler flags (or aliases)
- At the very least, debug/release

Basic workflow, this is evolving as I design and plan things:

1. Check the modified status of src. If has been modified, proceed
2. Open the previous build file if exists
3. Cache every file in the source tree with its modified status


### Run
- Build and then execute
- Required: Figure out a way to easily tell if anything has been modified 

### Install
Alias for add —global

### Uninstall
Alias for rm —global 

### Add
Add a package to the project. Point of concern: do we default to keeping a global bank of packages? or everything local as default?

Workflow:
1. First, check to see if package is installed globally. Maybe if so ask the user if they want to use this? Also: add a flag
2. Queries the repo for the package, return not found if not found
3. If found, fetches the package and compiles it? (This behavior functions differently from cargo, which compiles external packages at build time. I think this may be better for C++ though) Include path and library linkage are all managed. Saves a hash of the build version for update command (below)
