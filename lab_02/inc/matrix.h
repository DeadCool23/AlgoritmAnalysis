#ifndef __MATRIX_H__
#define __MATRIX_H__

#include <stddef.h>

#define MAX_MATRIX_SIZE 300

typedef size_t size_type;
typedef double value_type;
typedef value_type data_t[MAX_MATRIX_SIZE][MAX_MATRIX_SIZE];

typedef struct {
    size_type rows, cols;
    value_type **data;
} matrix_t;

typedef matrix_t *(*matrix_alg_t)(const matrix_t * const, const matrix_t * const);

value_type **alloc_matrix(const size_type rows, const size_type cols);

matrix_t *init_matrix_by_size(const size_type rows, const size_type cols);
matrix_t *init_matrix_by_array(const size_type rows, const size_type cols, data_t data);
matrix_t *init_matrix_by_size_with_rep_el(const size_type rows, const size_type cols, const value_type el);

matrix_t *std_matrix_mult(const matrix_t * const mtr1, const matrix_t * const mtr2);
matrix_t *winograd_matrix_mult(const matrix_t * const mtr1, const matrix_t * const mtr2);
matrix_t *optimized_winograd_matrix_mult(const matrix_t * const mtr1, const matrix_t * const mtr2);

matrix_t *matrix_input();
void matrix_print(const matrix_t *mtr);

void free_matrix(matrix_t *matrix);
void free_data(value_type **data, const size_type rows);

#endif // __MATRIX_H__