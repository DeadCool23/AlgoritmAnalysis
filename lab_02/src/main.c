#include <stdio.h>

#include "opts.h"
#include <stdint.h>

#ifdef TEST
#include "tests.h"
#include <stdlib.h>
#endif // TEST

uint64_t main(int argc, char **argv) {
#ifdef TEST
    int total_tests = 0;
    int num_failed = 0;

    test(&num_failed, &total_tests);
    int num_succes = total_tests - num_failed;
    return ((uint64_t)num_failed << 32) | num_succes;
#else
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
#endif // TEST
}
