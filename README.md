<a href="https://jabcode.org">
    <img src="docs/img/jabcode_logo.png" alt="JAB logo" title="JAB" align="right" height="80" />
</a>

# JAB Code

JAB Code (Just Another Bar Code) is a high-capacity 2D color bar code, which can encode more data than traditional black/white codes. This repository contains a library for reading and writing JAB codes, along with sample applications. A demo webinterface is available at https://jabcode.org.

## Introduction

JAB Code is a color two-dimensional matrix symbology whose basic symbols are made of colorful square modules arranged in either square or rectangle grids. JAB Code has two types of basic symbols, named as primary symbol and secondary symbol. A JAB Code contains one primary symbol and optionally multiple secondary symbols. Primary symbol contains four finder patterns located at the corners of the symbol, while secondary symbol contains no finder pattern. A secondary symbol can be docked to a primary symbol or another docked secondary symbol in either horizontal or vertical direction. JAB Code can encode from small to large amount of data correlated to user-specified percentages of error correction.

A demo webinterface is [provided](https://jabcode.org/create) to evaluate the library:
[![JAB Demo Webinterface](docs/img/jabcode_interface.png)](https://jabcode.org/create)

## Project Structure
    .
    ├── docs                  # Documentation
    └── src                   # Source code
         ├── jabcode          # JAB Code core library
         ├── jabcodeReader    # JAB Code reader application
         └── jabcodeWriter    # JAB Code writer application

## Build Instructions
The JAB Code core library, reader and writer applications are written in C (C11) and tested under Ubuntu 14.04 with gcc 4.8.4 and GNU Make 3.8.1. 

Follow the following steps to build the core library and applications. 

Step 1: Build the JAB Code core library by running make command in `src/jabcode`.

Step 2: Build the JAB Code writer by running make command in `src/jabcodeWriter`.

Step 3: Build the JAB Code reader by running make command in `src/jabcodeReader`.

The built library can be found in `src/jabcode/build`. The built reader and writer applications can be found in `src/jabcodeReader/bin` and `src/jabcodeWriter/bin`.

## Usage
The usage of jabcodeWriter and jabcodeReader can be obtained by running the programs with the argument `--help`.

##### jabcodeReader
run `jabcodeReader --help` for the detailed usage

##### jabcodeWriter
run `jabcodeWriter --help` for the detailed usage

## Build Windows DLL
Steps to build a DLL library on Windows System.

Step 1: Download and install mingw-w64 from http://mingw-w64.org/.

Step 2: For 64-bit Windows, Step 3 can be skipped, because the necessary libraries are already prebuilt. 

Step 3: For 32-bit Windows, download the source codes of libpng, libtiff and zlib from the following sites. 
			http://www.libpng.org/
			http://www.libtiff.org/
			https://www.zlib.net/
		Build the static libraries for libpng, libtiff and zlib in mingw-64 and put the built libraries in the folder "jabcode/lib/win64" to replace the 64-bit version. 
   
Step 4: Rename the file "Makefile.win" to "Makefile" in the folder "jabcode".

Step 5: Run "make" in mingw-64 in the folder "jabcode".

Step 6: Find the built DLL library "libjabcode.dll" in the folder "jabcode/build".

## Documentation
* The API documentation is available at [Documentation](https://jabcode.github.io/jabcode/)
* The technical specification of the barcode is available as [BSI TR03137 - Part 2](https://www.bsi.bund.de/EN/Service-Navi/Publications/TechnicalGuidelines/TR03137/BSITR03137.html)

(*Note that JabCode is under development for ISO standardization, the source code will be updated from time to time. The documentation above may therefore differ from the source code.*)
