# SuiteScript Generator

## Overview
This project was written to automate the boilerplate involved in creating valid SuiteScript files.

## Features
The CLI does the following:
- Create a new file
- Write the Copyright message to the file
- Write the SuiteScript script type, if supplied
- Write the SuiteScript API version
- Write the AMD Module Definition with SuiteScript modules (N/*), if supplied

## Options
| Short | Long | Description | Default | Constraints |
| ----- | ---- | ----------- | ------- | ----------- |
|  -h   | --help | Displays the help message | N/A | The extension must be `.js` |
|  -f   | --filename | The filename to be created | N/A | Required |
|  -t   | --stype | The type of SuiteScript to be created | No type | Must be a valid SuiteScript type |
|  -v   | --version | The SuiteScript API version to use | 2.1 | Must be either 1, 2, 2.x, or 2.1 |
|  -m   | --modules | The SuiteScript API modules to import | No modules | N/A |


## Usage

To create a simple file skeleton:
`suitescript -f myFile.js` or `suitescript --filename myFile.js`

To create a file with a specific version:
`suitescript -f myFile.js -v 2.1`

To create a file for a specific script type:
`suitescript -f myFile.js -t MapReduce`

To create a file with imported modules:
`suitescript -f myFile.js -m record search`
