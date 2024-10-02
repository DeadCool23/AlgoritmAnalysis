#ifndef __OPTS_H__
#define __OPTS_H__

#include <stdio.h>

typedef struct {
    enum {
        INTERACTIVE,
        MEASURES,
        HELP,
        UNKNOWN,
        NO_ARGS
    } opt;
    FILE *f;
} options_t;

options_t get_options(int argc, char **argv);

void print_help();
void print_unknown();

int interactive_mult();
int get_measures(FILE *f);

#endif // __OPTS_H__