#include <stdio.h>

#include "opts.h"

int main(int argc, char **argv) {
    options_t op = get_options(argc, argv);
    if (op.opt == HELP) {
        print_help();
        return 0;
    } else if (op.opt == UNKNOWN) {
        print_unknown();
        return -1;
    } else if (op.opt == INTERACTIVE) {
        return interactive_mult();
    } else if (op.opt == MEASURES) {
        if (!op.f) {
            printf("ERROR: Can't open file");
            return -1;
        }
        return get_measures(op.f);
    } else {
        printf("ERROR: No option input\n");
        print_unknown();
        return -1;
    }
}
