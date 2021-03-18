# Transpire

*NO ENGLISH? NO PROBLEM!*

## Project Description

This project sets up an open-source-contribution environemnt where users anywhere around the globe can contribute translations of tokens in various different languages.

Our project will automatically keep track of new translation contributions and integrate it into our "transpiler" which will allow you to code in any human-spoken-language and generate source code for any programming langauge (given that someone committed translations of it to the repository).

## How does this all work?

There are two components to this project:

1. The parser
2. The translations

This project is the parser. It's main role is to fetch the up-to-date translations from our servers and carry out the translation process of the program.

The translations part of our project is a completely different repository: 

https://github.com/OcEaNvS/trnpkgs


The trnpkgs will be open sourced to allow pull requests any user to make translations from their own language to any programming language.

Every time someone builds our program, it will fetch a tarball of the most stable translation mapping and store it in the local machine.

## Documentation

**[LINK HERE](https://aldiyarablyazov.github.io/Transpire)**


## Directory Structure
[TODO]
```

## How to Build

1) Install dependencies (if any)

2) Clone the repo: git clone https://github.com/aldiyarablyazov/Transpire/

3) Run this command: ``` [TODO] ```

3) If everything goes according to plan, you should see an output akin to the following:

```bash

> Task :curlTranslations
  % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
                                 Dload  Upload   Total   Spent    Left  Speed
100   356  100   356    0     0    726      0 --:--:-- --:--:-- --:--:--   725

BUILD SUCCESSFUL in 3s
8 actionable tasks: 8 executed
``` 

4) It is important to note that a folder called "translations" should be generated under the project root. If it doesn't, please create it yourself with `mkdir`

## How to Run

Our program's argument has the following strucutre:

```bash
usage: transpire [-h] --lang {python,c,java} --source {en,fr,jp}
                 [--target {en}] [--update] [--verbose] FILE [FILE ...]

Translates program written in native language to compilable code.

positional arguments:
  FILE                   A file to translate to other language

named arguments:
  -h, --help             show this help message and exit
  --lang {python,c,java}, -l {python,c,java}
                         The programming language used here.
  --source {en,fr,jp}, -s {en,fr,jp}
                         The source file natural language.
  --target {en}, -t {en}
                         The spoken language to translate the file to.
  --update, -u           Whether to update the JSON if already downloaded.
  --verbose, -v          Whether the program should be verbose.
```

Gradle automatically runs the transpire app with the arguments stated inside --args. The above example will take the the java file (`--lang java`) called sampleFile.java, read it as japanese (`--source jp`), and map it to the standard java syntax (`--target en`) using the respective json. The `-u` argument always fetches the most up-to-date translation and caches it into local machine. If you do not have internet connection, drop the `-u`.

One can also run it using the `transpire` binary

``` bash
./transpire sampleFile.java -u --lang java --source jp --target en
```

