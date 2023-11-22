#include <stdio.h>
#include <assert.h>
#include <stdbool.h>
#include <unistd.h>
#include <string.h>

// Function prototypes for Rust functions
extern double file_creation_date(const char *path);
extern double file_modification_date(const char *path);
extern double file_access_date(const char *path);
extern double file_size(const char *path);
extern bool file_exists(const char *path);
extern bool file_is_directory(const char *path);

// Function to create a temporary file for testing
const char *get_file() {
  // Get the current dir and append the Cargo.toml file name
  char cwd[1024];
  getcwd(cwd, sizeof(cwd));
  char *path = strcat(cwd, "/Cargo.toml");

  // Return the full path
  return path;
}

int main() {
    const char *path = get_file();

    // Test file_creation_date
    double creation_date = file_creation_date(path);
    printf("File Creation Date: %f\n", creation_date);
    assert(creation_date > 0.0); // Assuming a positive creation time on success

    // Test file_modification_date
    double modification_date = file_modification_date(path);
    printf("File Modification Date: %f\n", modification_date);
    assert(modification_date > 0.0); // Assuming a positive modification time on success

    // Test file_access_date
    double access_date = file_access_date(path);
    printf("File Access Date: %f\n", access_date);
    assert(access_date > 0.0); // Assuming a positive access time on success

    // Test file_size
    double size = file_size(path);
    printf("File Size: %f\n", size);
    assert(size >= 0.0); // Assuming a non-negative file size on success

    // Test file_exists
    bool exists = file_exists(path);
    printf("File Exists: %s\n", exists ? "true" : "false");
    assert(exists);

    // Test file_is_directory
    bool is_directory = file_is_directory(path);
    printf("Is Directory: %s\n", is_directory ? "true" : "false");
    assert(is_directory);

    printf("All tests passed!\n");

    return 0;
}