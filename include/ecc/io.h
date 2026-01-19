#ifndef ECC_IO_H
#define ECC_IO_H

/**
 * Read the contents of a file to a string.
 *
 * Return a heap-allocated buffer containing all text from the file specified by
 * `path`. Once the caller is done with the buffer, it should be deallocated
 * using `free`.
 *
 * If an error occurs, this function returns `NULL`. Query `errno` for more
 * information.
 */
char *ecc_io_read_file(const char *path);

#endif
