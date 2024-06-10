
# Luoxidant Lua Parser

Luoxidant's parser is a parser for a Lua language, designed to interpret and compile scripts written in Lua. This document outlines the architecture and components of the parser, focusing on its design principles and key functionalities.

## Overview

The Luoxidant parser is structured around a series of modules, each responsible for a distinct aspect of the parsing process. From lexical analysis to syntax tree construction, the parser aims to provide a robust foundation for executing Lua-like scripts efficiently.

## Key Features

- Lexical Analysis: Tokenizes input scripts into meaningful symbols.
- Syntax Analysis: Constructs an abstract syntax tree (AST) from tokens, enforcing the language's grammar rules.
- Error Handling: Implements sophisticated error reporting and recovery mechanisms to assist developers in diagnosing and fixing script issues.
- Extensibility: Designed with extensibility in mind, allowing for easy addition of new language features and constructs.

## Architecture

The parser is divided into several core components:

- Lexer: Breaks down the input script into tokens.
Parser: Builds an AST from the stream of tokens produced by the lexer.
- AST Nodes: Represents the syntactic structure of the script.
- Error Reporting: Collects and reports errors encountered during parsing.

## Helper Functions in common.rs

The common.rs module contains a collection of utility functions and constants that aid in the parsing process. These helpers facilitate token matching, operator detection, and character conversion tasks.

### Token Matching

- either: Checks if the current token matches any of the specified kinds. Useful for identifying tokens among a set of possibilities.
- at: Verifies if the current token exactly matches a specified TokenKind. Specialized for single-token matching scenarios.
- must_be_either: Enforces that the current token matches one of the specified kinds, reporting an error if not.

### Operator Detection

- is_unary: Identifies if the current token represents a unary operator, facilitating the parsing of unary expressions.
- is_binary: Detects if the current token is a binary operator, crucial for parsing binary expressions.

### Character Digit Conversion

- from_digit: Converts an ASCII digit character to its numeric value, supporting the parsing of numeric literals.
- from_hex_digit: Translates an ASCII hexadecimal digit character to its numeric equivalent, enabling the interpretation of hexadecimal literals.

These utilities play a pivotal role in simplifying the parsing logic and enhancing the parser's ability to accurately interpret scripts.