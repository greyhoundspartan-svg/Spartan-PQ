// Stub implementations for Spartan2 when Labrador library cannot be compiled
// This provides minimal functions to make the linking work

#include <stdlib.h>
#include <stdint.h>

// Stub implementations - these don't do anything but allow linking
void* malloc(size_t size) {
    return NULL;
}

void free(void* ptr) {
    // Do nothing
}

void* realloc(void* ptr, size_t size) {
    return NULL;
}

void* calloc(size_t nmemb, size_t size) {
    return NULL;
}

// Add any other minimal stubs needed for linking
// This is a minimal approach to get Spartan2 building
