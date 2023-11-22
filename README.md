# File Metadata
Tiny baby library for getting file metadata. Originally written to work for a GameMaker game a friend is creating.

# Table of Contents
* [Installation](#installation)
  * [Alternative Installation](#alternative-installation)
* [Documentation](#documentation)
  * [General Notes](#general-notes)
  * [`filemeta_creation_date(path)`](#filemeta_creation_datepath)
  * [`filemeta_modification_date(path)`](#filemeta_modification_datepath)
  * [`filemeta_access_date(path)`](#filemeta_access_datepath)
  * [`filemeta_size(path)`](#filemeta_sizepath)
  * [`filemeta_exists(path)`](#filemeta_existspath)
  * [`filemeta_is_directory(path)`](#filemeta_is_directorypath)

# Installation

1. Download the `.yymps` file from the [latest release](https://github.com/SpikeHD/file-metadata/releases/latest).
3. That's it! You can now import the package into your project by dragging it into your GameMaker window.

## Alternative Installation
You can also use the dynamic library itself by extracting the `.yymps` file as a zip file, and adding the `.dll`/`.dylib`/`.so` file found within to your project manually by following [this guide](https://forum.gamemaker.io/index.php?threads/basic-extension-creation.42662/) (skip the bits about Visual Studio and such).

# GameMaker Usage Documentation

## General Notes

* If a function encounters an error, it will return -1.0 and log to stderr.
* Timestamps are in seconds, not milliseconds.
* Internally, functions are prefixed with `file_` rather than `filemeta_`, so if you are using the [alternative installation](#alternative-installation) method, the mapping is `filemeta_creation_date` -> `file_creation_date`, etc.

## `filemeta_creation_date(path)`

Returns the creation date of the file at `path` as a UNIX timestamp in seconds.

## `filemeta_modification_date(path)`

Returns the modification date of the file at `path` as a UNIX timestamp in seconds.

## `filemeta_access_date(path)`

Returns the access date of the file at `path` as a UNIX timestamp in seconds.

## `filemeta_size(path)`

Returns the size of the file at `path` in bytes.

## `filemeta_exists(path)`

Returns `true` (1) if the file at `path` exists, `false` (0) otherwise.

## `filemeta_is_directory(path)`

Returns `true` (1) if the file at `path` is a directory, `false` (0) otherwise.
