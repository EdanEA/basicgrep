# basicgrep
## About
I was going through an old Rust book and there was an excersise for making a basic grep program. This follows the same general methodologies of the excersice, but is expanded upon to support flags and more complex grep functionalities, like reading from multiple files and regex support.

## How to Start Using
Clone, build, and run the compiled program.

## Usage
The program takes arguments in the following structure:
`basicgrep [-c] [-i] ([-e pattern] | [query string]) <file(s)>`
- `-c` is a flag for taking count of matches
- `-i` is for ignoring character case
- `-e` is to enable regex search and is immediately followed by the regex pattern in quotation marks
- if `-e` is not used a query string is provided and lastly the filename(s)
