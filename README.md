# File Metadata
Baby lib for getting file metadata. Originally written to work for a GameMaker game a friend is creating.

# Installation

1. Download the latest package version for your target platform from [Actions](https://github.com/SpikeHD/file-metadata/actions).
2. Extract the `.yymps` file from the archive.
3. That's it! You can now import the package into your project. 

# Documentation

## `file_creation_date(path)`

Returns the creation date of the file at `path` as a UNIX timestamp in seconds.

## `file_modification_date(path)`

Returns the modification date of the file at `path` as a UNIX timestamp in seconds.

## `file_access_date(path)`

Returns the access date of the file at `path` as a UNIX timestamp in seconds.

## `file_size(path)`

Returns the size of the file at `path` in bytes.

## `file_exists(path)`

Returns `true` (1) if the file at `path` exists, `false` (0) otherwise.

## `file_is_directory(path)`

Returns `true` (1) if the file at `path` is a directory, `false` (0) otherwise.