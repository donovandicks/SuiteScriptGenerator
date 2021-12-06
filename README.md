# SuiteScript Generator

[![Contributor Covenant](https://img.shields.io/badge/Contributor%20Covenant-2.0-4baaaa.svg)](code_of_conduct.md)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## Overview

This project is designed to help aid in the development of NetSuite SuiteScript projects. Using the
CLI, you can automatically create new SuiteScripts with the AMD module boilerplate included.

NOTE: This project is __NOT__ intended to replace the SuiteCloud Developer Framework and the project
structure that is created using that tool. This project is intended to be used __with__ the SDF to
generate the files specific to your project, after the SDF generates the project strcture.

## Features

The CLI does the following:

- Create a new file
- Write a Copyright message to the file, if supplied
- Write the SuiteScript script type, if supplied
- Write the SuiteScript API version
- Write the AMD Module Definition with SuiteScript modules (N/*), if supplied

## Options

| Short | Long         | Description                                         | Default      | Constraints                                 |
| ----- | ------------ | --------------------------------------------------- | ------------ | ------------------------------------------- |
| -h    | --help       | Displays the help message                           | N/A          | N/A                                         |
| -f    | --filename   | The filename to be created                          | N/A          | Required, File extension must be `.js`      |
| -c    | --copyright  | The text file where the copyright message is stored | No copyright | File extension must be `.txt`               |
| -s    | --scripttype | The type of SuiteScript to be created               | No type      | Must be a valid SuiteScript type            |
| -a    | --apiversion | The SuiteScript API version to use                  | 2.1          | Must be either 2.0, 2.x, or 2.1             |
| -m    | --modules    | The SuiteScript API modules to import               | No modules   | Must be a valid NetSuite SuiteScript module |

## Usage

The output files from the following commands are visible in the [examples](examples) directory.

To create a simple file skeleton:
`suitescript -f basic.js` or `suitescript --filename basic.js`

To create a file with a specific version:
`suitescript -f versioned.js -v 2.0`

To create a file for a specific script type:
`suitescript -f typed.js -t MapReduce`

To create a file with imported modules:
`suitescript -f imports.js -m record search`

To create a file with a copyright doc comment:
`suitescript -f copyright.js -c copyright.txt`

And any combination:
`suitescript -f combo.js -c copyright.txt -v 2.x -m record search -t client`

## References

Please refer to the NetSuite guides for valid SuiteScript types and modules, and for more information
on developing your SuiteScript files and projects.

Guides:

- [SuiteScript Developer Guide](https://docs.oracle.com/cloud/latest/netsuitecs_gs/NSCDG/NSCDG.pdf)
- [SuiteScript 2.0 API Reference](https://docs.oracle.com/cloud/latest/netsuitecs_gs/NSAPI/NSAPI.pdf)
- [SuiteCloud Development Framework](https://docs.oracle.com/cloud/latest/netsuitecs_gs/NSCDF/NSCDF.pdf)

## Roadmap

- [ ] Support reading an input file for generating entire projects
- [ ] Support custom modules with local or absolute paths
- [ ] Support custom names for module arguments
- [ ] Support skeletons for known SuiteScript entry points when applicable

## Contributing

Please open an issue before making a pull request.

Please update tests as appropriate.

## License

[MIT](https://choosealicense.com/licenses/mit/)
