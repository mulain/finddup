# finddup — Duplicate File Finder in Rust

`finddup` scans a directory and finds duplicate files by size and content hash.

##  Features
-  Recursive directory scan
-  Multithreaded hashing (Rayon)
-  Groups files by hash
-  Colorized terminal output
-  JSON export option

## Usage
```bash
finddup <directory> [options]

Options:
  --min-size <bytes>   Ignore files smaller than X bytes
  --threads <n>        Set number of threads
  --json               Output results as JSON
```
 
## Learnings
- clap
command line argument parser
