# fileservant

A optimized HTTP file server written in Rust.

## Install

```bash
sudo add-apt-repository ppa:stapat/fileservant
sudo apt update
sudo apt install fileservant
```

## Usage

Serve the current directory:

```bash
fileservant
```

Choose a port:

```bash
fileservant -p 8080
```

Choose an address:

```bash
fileservant -i 127.0.0.1 -p 8080
```
