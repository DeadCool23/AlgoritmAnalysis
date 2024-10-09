#include "tests.h"
#include "matrix.h"

// Одно из матриц неинициализированна
START_TEST(incorrect_matrix_ptr_in_std_mult) {
#define M 3
#define K 4
    data_t b = {{2, 0, 0}, {1, 2, 3}, {0, 1, 2}, {1, 2, 2}};
    matrix_t *mb = init_matrix_by_array(K, M + 1, b);

    matrix_t *res = std_matrix_mult(NULL, mb);

    ck_assert_ptr_eq(res, NULL);

    free_matrix(mb);
#undef M
#undef K
} END_TEST

// Некорректные размеры матриц для умножения
START_TEST(incorrect_size_std_matrix_mult) {
#define N 2
#define M 3
#define K 4
    data_t a = {{1, 2, 3}, {4, 5, 6}};
    data_t b = {{2, 0, 0}, {1, 2, 3}, {0, 1, 2}, {1, 2, 2}};
    matrix_t *ma = init_matrix_by_array(N, M, a);
    matrix_t *mb = init_matrix_by_array(K, M + 1, b);

    matrix_t *res = std_matrix_mult(ma, mb);

    ck_assert_ptr_eq(res, NULL);

    free_matrix(ma);
    free_matrix(mb);
#undef N
#undef M
#undef K
} END_TEST

// Умножение матрицы-строки на квадратную матрицу
START_TEST(row_std_matrix_mult) {
#define N 2
    data_t a = {{1, 2}};
    data_t b = {{3, 4}, {5, 6}};
    matrix_t *ma = init_matrix_by_array(1, N, a);
    matrix_t *mb = init_matrix_by_array(N, N, b);

    matrix_t *res = std_matrix_mult(ma, mb);

    data_t real = {{13, 16}};
    matrix_t *expected = init_matrix_by_array(1, N, real);

    ck_assert_ptr_ne(res, NULL);
    ck_assert_int_eq(ck_assert_matrix_double_eq(res->data, expected->data, res->rows, res->cols), 0);

    free_matrix(ma);
    free_matrix(mb);
    free_matrix(res);
    free_matrix(expected);
#undef N
} END_TEST

// Умножение квадратной матрицы на матрицу-столбец
START_TEST(square_matrix_column_std_mult) {
#define N 2
    data_t a = {{1, 2}, {3, 4}};
    data_t b = {{2}, {1}};
    matrix_t *ma = init_matrix_by_array(N, N, a);
    matrix_t *mb = init_matrix_by_array(N, 1, b);

    matrix_t *res = std_matrix_mult(ma, mb);

    data_t real = {{4}, {10}};
    matrix_t *expected = init_matrix_by_array(N, 1, real);

    ck_assert_ptr_ne(res, NULL);
    ck_assert_int_eq(ck_assert_matrix_double_eq(res->data, expected->data, res->rows, res->cols), 0);

    free_matrix(ma);
    free_matrix(mb);
    free_matrix(res);
    free_matrix(expected);
#undef N
} END_TEST

// Умножение матрицы-столбца на матрицу-строку
START_TEST(column_matrix_row_std_mult) {
#define N 2
    data_t a = {{1, 2}};
    data_t b = {{2}, {1}};
    matrix_t *ma = init_matrix_by_array(1, N, a);
    matrix_t *mb = init_matrix_by_array(N, 1, b);

    matrix_t *res = std_matrix_mult(ma, mb);

    data_t real = {{4}};
    matrix_t *expected = init_matrix_by_array(1, 1, real);

    ck_assert_ptr_ne(res, NULL);
    ck_assert_int_eq(ck_assert_matrix_double_eq(res->data, expected->data, res->rows, res->cols), 0);

    free_matrix(ma);
    free_matrix(mb);
    free_matrix(res);
    free_matrix(expected);
#undef N
} END_TEST

// Умножение квадратной матрицы на единичную
START_TEST(square_matrix_identity_std_mult) {
#define N 2
    data_t a = {{1, 2}, {3, 4}};
    data_t b = {{1, 0}, {0, 1}};
    matrix_t *ma = init_matrix_by_array(N, N, a);
    matrix_t *mb = init_matrix_by_array(N, N, b);

    matrix_t *res = std_matrix_mult(ma, mb);

    matrix_t *expected = init_matrix_by_array(N, N, a);

    ck_assert_ptr_ne(res, NULL);
    ck_assert_int_eq(ck_assert_matrix_double_eq(res->data, expected->data, res->rows, res->cols), 0);

    free_matrix(ma);
    free_matrix(mb);
    free_matrix(res);
    free_matrix(expected);
#undef N
} END_TEST

// Умножение прямоугольных матриц с квадратным результатом
START_TEST(rectangular_std_matrix_mult_res_square) {
#define N 2
#define M 3
    data_t a = {{1, 2, 3}, {4, 5, 6}};
    data_t b = {{2, 0}, {1, 2}, {0, 1}};
    matrix_t *ma = init_matrix_by_array(N, M, a);
    matrix_t *mb = init_matrix_by_array(M, N, b);

    matrix_t *res = std_matrix_mult(ma, mb);

    data_t real = {{4, 7}, {13, 16}};
    matrix_t *expected = init_matrix_by_array(N, N, real);

    ck_assert_ptr_ne(res, NULL);
    ck_assert_int_eq(ck_assert_matrix_double_eq(res->data, expected->data, res->rows, res->cols), 0);

    free_matrix(ma);
    free_matrix(mb);
    free_matrix(res);
    free_matrix(expected);
#undef N
#undef M
} END_TEST

// Умножение прямоугольных матриц с прямоугольным результатом
START_TEST(rectangular_std_matrix_mult_res_rectangular) {
#define N 2
#define M 3
    data_t a = {{1, 2, 3}, {4, 5, 6}};
    data_t b = {{2, 0, 3}, {1, 2, 3}, {0, 1, 3}};
    matrix_t *ma = init_matrix_by_array(N, M, a);
    matrix_t *mb = init_matrix_by_array(M, M, b);

    matrix_t *res = std_matrix_mult(ma, mb);

    data_t real = {{4, 7, 18}, {13, 16, 45}};
    matrix_t *expected = init_matrix_by_array(N, M, real);

    ck_assert_ptr_ne(res, NULL);
    ck_assert_int_eq(ck_assert_matrix_double_eq(res->data, expected->data, res->rows, res->cols), 0);

    free_matrix(ma);
    free_matrix(mb);
    free_matrix(res);
    free_matrix(expected);
#undef N
#undef M
} END_TEST

// Умножение квадратных матриц первого порядка
START_TEST(first_ord_square_std_matrix_mult) {
#define N 1
    data_t a = {{2}};
    data_t b = {{3}};
    matrix_t *ma = init_matrix_by_array(N, N, a);
    matrix_t *mb = init_matrix_by_array(N, N, b);

    matrix_t *res = std_matrix_mult(ma, mb);

    data_t real = {{6}};
    matrix_t *expected = init_matrix_by_array(N, N, real);

    ck_assert_ptr_ne(res, NULL);
    ck_assert_int_eq(ck_assert_matrix_double_eq(res->data, expected->data, res->rows, res->cols), 0);

    free_matrix(ma);
    free_matrix(mb);
    free_matrix(res);
    free_matrix(expected);
#undef N
} END_TEST

// Умножение двух квадратных матриц
START_TEST(square_std_matrix_mult) {
#define N 2
    data_t a = {{1, 2}, {3, 4}};
    data_t b = {{2, 0}, {1, 2}};
    matrix_t *ma = init_matrix_by_array(N, N, a);
    matrix_t *mb = init_matrix_by_array(N, N, b);

    matrix_t *res = std_matrix_mult(ma, mb);

    data_t real = {{4, 4}, {10, 8}};
    matrix_t *expected = init_matrix_by_array(N, N, real);

    ck_assert_ptr_ne(res, NULL);
    ck_assert_int_eq(ck_assert_matrix_double_eq(res->data, expected->data, res->rows, res->cols), 0);

    free_matrix(ma);
    free_matrix(mb);
    free_matrix(res);
    free_matrix(expected);
#undef N
} END_TEST


Suite* std_mult_suite(void) {
    Suite *s;
    TCase *tc_neg;
    TCase *tc_pos;

    s = suite_create("\n___STD_MULT___");

    // Отрицательные тесты
    tc_neg = tcase_create(" negatives ");
    tcase_add_test(tc_neg, incorrect_matrix_ptr_in_std_mult);
    tcase_add_test(tc_neg, incorrect_size_std_matrix_mult);
    suite_add_tcase(s, tc_neg);

    // Положительные тесты
    tc_pos = tcase_create(" positives ");
    tcase_add_test(tc_pos, row_std_matrix_mult);
    tcase_add_test(tc_pos, square_matrix_column_std_mult);
    tcase_add_test(tc_pos, column_matrix_row_std_mult);
    tcase_add_test(tc_pos, rectangular_std_matrix_mult_res_rectangular);
    tcase_add_test(tc_pos, square_matrix_identity_std_mult);
    tcase_add_test(tc_pos, rectangular_std_matrix_mult_res_square);
    tcase_add_test(tc_pos, first_ord_square_std_matrix_mult);
    tcase_add_test(tc_pos, square_std_matrix_mult);

    suite_add_tcase(s, tc_pos);

    return s;
}
