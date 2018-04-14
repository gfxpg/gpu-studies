#ifndef IO_H
#define IO_H

#include <stdio.h>
#include <stdlib.h>

static inline char* read_file(const char* filename) {
    FILE* file;
    size_t file_len;
    char* file_contents;

    file = fopen(filename, "rb");
    if (!file) return NULL;

    fseek(file, 0, SEEK_END);
    file_len = (size_t) ftell(file);
    rewind(file);

    file_contents = (char*) malloc(file_len + 1);

    fread(file_contents, sizeof(char), file_len, file);
    fclose(file);

    file_contents[file_len] = '\0';
    return file_contents;
}

#endif //IO_H
