#ifndef __TESTS_H__
#define __TESTS_H__

#include "tests.h"

#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>

void test(int *num_failed, int *total_tests) {
    SRunner *runner;

    Suite *(*unit_tests[])(void) = {
        std_mult_suite,
        winograd_mult_suite,
        opt_winograd_mult_suite,
    };
    size_t tests_cnt = sizeof(unit_tests) / sizeof(*unit_tests);

    *total_tests = 0;
    *num_failed = 0;

    for (size_t i = 0; i < tests_cnt; i++) {
        printf("\n\n");
        runner = srunner_create(unit_tests[i]());
        srunner_run_all(runner, CK_MINIMAL);

        // Получаем количество всех выполненных тестов
        *total_tests += srunner_ntests_run(runner);

        // Получаем количество проваленных тестов
        *num_failed += srunner_ntests_failed(runner);
        srunner_free(runner);
    }
}

#define EPS 1e-8
int ck_assert_matrix_double_eq(double **X, double **Y, size_t rows, size_t cols) {
    int diff = 0;
    for (size_t i = 0; i < rows && !diff; i++)
        for (size_t j = 0; j < cols && !diff; j++)
            diff = (fabs(X[i][j] - Y[i][j]) < EPS) ? 0 : 1;
    
    return diff;
}

#endif // __TESTS_H__