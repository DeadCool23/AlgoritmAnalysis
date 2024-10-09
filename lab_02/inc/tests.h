#ifndef __CHECK_MATH__
#define __CHECK_MATH__

#include <check.h>

int ck_assert_matrix_double_eq(double **X, double **Y, size_t rows, size_t cols);

/**
 * @brief Модульное тестирование стандартного произведения матриц
 *
 * @return Набор тестов
 */
Suite* std_mult_suite(void);

/**
 * @brief Модульное тестирование произведения матриц алгоритмом Винограда
 *
 * @return Набор тестов
 */
Suite* winograd_mult_suite(void);

/**
 * @brief Модульное тестирование произведения матриц алгоритмом Винограда
 *
 * @return Набор тестов
 */
Suite* opt_winograd_mult_suite(void);

void test(int *num_failed, int *total_tests);

#endif  //__CHECK_MATH__