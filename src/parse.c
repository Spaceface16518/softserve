#include <string.h>

const char * parse(const char* line)
{
    // Find out where the path is in the request
    const char *start_of_path = strchr(line, ' ') + 1;
    const char *end_of_path = strchr(start_of_path, '?');

    // Allocate the right amount of memory
    char path[end_of_path - start_of_path];

    // Copy the strings into the created memory
    strncpy(path, start_of_path,  end_of_path - start_of_path);

    // Append a null terminator
    path[sizeof(path)] = 0;

    return path;
}
