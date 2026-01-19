#include <stdio.h>
#include <stdlib.h>

static size_t file_size(FILE *file) {
  fseek(file, 0, SEEK_END);
  size_t size = ftell(file);
  rewind(file);
  return size;
}

char *ecc_io_read_file(const char *path) {
  FILE *file = fopen(path, "r");
  if (file == NULL) {
    return NULL;
  }

  size_t size = file_size(file);
  char *buffer = malloc(size + 1);
  if (buffer == NULL) {
    fclose(file);
    return NULL;
  }

  size_t bytes_read = fread(buffer, 1, size, file);
  if (bytes_read < size) {
    free(buffer);
    fclose(file);
    return NULL;
  }

  buffer[bytes_read] = '\0';
  fclose(file);
  return buffer;
}
