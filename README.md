# SuiteScript Generator

## Overview
This project was written to automate the boilerplate involved in creating valid SuiteScript files.

## Features
The CLI does the following:
- Create a new file
- Write a Copyright message to the file, if supplied
- Write the SuiteScript script type, if supplied
- Write the SuiteScript API version
- Write the AMD Module Definition with SuiteScript modules (N/*), if supplied

## Options
| Short | Long | Description | Default | Constraints |
| ----- | ---- | ----------- | ------- | ----------- |
|  -h   | --help | Displays the help message | N/A | N/A |
|  -f   | --filename | The filename to be created | N/A | Required, File extension must be `.js` |
|  -c   | --copyright | The text file where the copyright message is stored | No copyright | File extension must be `.txt` |
|  -t   | --stype | The type of SuiteScript to be created | No type | Must be a valid SuiteScript type |
|  -v   | --version | The SuiteScript API version to use | 2.1 | Must be either 2.0, 2.x, or 2.1 |
|  -m   | --modules | The SuiteScript API modules to import | No modules | N/A |

## Usage
The output files from the following commands are visible in the [examples](examples) directory.

To create a simple file skeleton:
`suitescript -f basic.js` or `suitescript --filename myFile.js`

To create a file with a specific version:
`suitescript -f versioned.js -v 2.0`

To create a file for a specific script type:
`suitescript -f typed.js -t MapReduce`

To create a file with imported modules:
`suitescript -f imports.js -m record search`

To create a file with a copyright doc comment:
`suitescript -f copyright.js -c copyright.txt`

## TODOs
- [X] Parameterize the copyright message
- [X] Pull out constants to another file
- [X] Add support for mangled names where possible i.e. mApReDuCe will still product MapReduceScript
- [ ] Support reading an input file for batch generating project files
- [X] Validate input for SuiteScript modules against list of supported/existing modules
- [ ] Support custom names for module arguments
- [ ] Support custom modules with local or absolute paths
- [ ] Unit tests
- [ ] Docs
