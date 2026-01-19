#include <stdio.h>
#include <stdlib.h>

#include "ecc/io.h"

int main(int argc, char **argv) {
  if (argc < 2) {
    fprintf(stderr, "%s: fatal error: no input files\n", argv[0]);
    fprintf(stderr, "compilation terminated.\n");
    return 1;
  }

  char *contents = ecc_io_read_file(argv[1]);
  if (contents == NULL) {
    fprintf(stderr, "%s: fatal error: %s: ", argv[0], argv[1]);
    perror(NULL);
    fprintf(stderr, "compilation terminated.\n");
    return 1;
  }

  printf("Compiling the file '%s' (among others)\n", argv[1]);
  printf("Contents of that file:\n%s\n", contents);

  free(contents); 

  return 0;
}
