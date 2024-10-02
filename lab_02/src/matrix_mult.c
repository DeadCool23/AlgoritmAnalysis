#include "matrix.h"

#include <stdlib.h>
#include <stdbool.h>

matrix_t *std_matrix_mult(const matrix_t * const mtr1, const matrix_t * const mtr2) {
    if (mtr1 == NULL || mtr2 == NULL) return NULL;
    if (mtr1->cols != mtr2->rows) return NULL;

    matrix_t *res = init_matrix_by_size(mtr1->rows, mtr2->cols);
    if (!res) return NULL;

    for (size_type i = 0; i < mtr1->rows; ++i) {
        for (size_type j = 0; j < mtr2->cols; ++j) {
            value_type sum = 0;
            for (size_type k = 0; k < mtr1->cols; ++k) {
                sum += mtr1->data[i][k] * mtr2->data[k][j];
            }
            res->data[i][j] = sum;
        }
    }
    return res;
}

matrix_t *winograd_matrix_mult(const matrix_t * const mtr1, const matrix_t * const mtr2) {
    if (mtr1 == NULL || mtr2 == NULL) return NULL;
    if (mtr1->cols != mtr2->rows) return NULL;

    size_type rows1 = mtr1->rows;
    size_type cols1 = mtr1->cols;
    size_type cols2 = mtr2->cols;

    matrix_t *res = init_matrix_by_size(rows1, cols2);
    if (!res) return NULL;

    value_type *row_factor = (value_type *)malloc(rows1 * sizeof(value_type));
    if (!row_factor) goto end;
    for (size_type i = 0; i < rows1; ++i) {
        row_factor[i] = 0;
        for (size_type j = 0; j < cols1 / 2; ++j) {
            row_factor[i] += mtr1->data[i][2 * j] * mtr1->data[i][2 * j + 1];
        }
    }

    value_type *col_factor = (value_type *)malloc(cols2 * sizeof(value_type));
    if (!col_factor) goto free_row_factor;
    for (size_type j = 0; j < cols2; ++j) {
        col_factor[j] = 0;
        for (size_type i = 0; i < cols1 / 2; ++i) {
            col_factor[j] += mtr2->data[2 * i][j] * mtr2->data[2 * i + 1][j];
        }
    }

    for (size_type i = 0; i < rows1; ++i) {
        for (size_type j = 0; j < cols2; ++j) {
            res->data[i][j] = -row_factor[i] - col_factor[j];
            for (size_type k = 0; k < cols1 / 2; ++k) {
                res->data[i][j] += (mtr1->data[i][2 * k    ] + mtr2->data[2 * k + 1][j]) *
                                   (mtr1->data[i][2 * k + 1] + mtr2->data[2 * k    ][j]);
            }
        }
    }

    if (cols1 & 1) {
        for (size_type i = 0; i < rows1; ++i) {
            for (size_type j = 0; j < cols2; ++j) {
                res->data[i][j] += mtr1->data[i][cols1 - 1] * mtr2->data[cols1 - 1][j];
            }
        }
    }

    free(col_factor);
    free(row_factor);
    
    return res;

    free_row_factor:
    free(row_factor);
    end:
    free_matrix(res);
    return res;
}

matrix_t *optimized_winograd_matrix_mult(const matrix_t * const mtr1, const matrix_t * const mtr2) {
    if (mtr1 == NULL || mtr2 == NULL) return NULL;
    if (mtr1->cols != mtr2->rows) return NULL;

    size_type rows1 = mtr1->rows;
    size_type cols1 = mtr1->cols;
    size_type cols2 = mtr2->cols;

    matrix_t *res = init_matrix_by_size(rows1, cols2);
    if (!res) return NULL;

    value_type *row_factor = (value_type *)malloc(rows1 * sizeof(value_type));
    if (!row_factor) goto end;
    for (size_type i = 0; i < rows1; ++i) {
        row_factor[i] = 0;
        for (size_type j = 0; j < cols1 / 2; ++j) {
            row_factor[i] -= mtr1->data[i][2 * j] * mtr1->data[i][2 * j + 1];
        }
    }

    value_type *col_factor = (value_type *)malloc(cols2 * sizeof(value_type));
    if (!col_factor) goto free_row_factor;
    for (size_type j = 0; j < cols2; ++j) {
        col_factor[j] = 0;
        for (size_type i = 0; i < cols1 / 2; ++i) {
            col_factor[j] -= mtr2->data[2 * i][j] * mtr2->data[2 * i + 1][j];
        }
    }

    bool flag = cols1 & 1;
    for (size_type i = 0; i < rows1; ++i) {
        for (size_type j = 0; j < cols2; ++j) {
            res->data[i][j] = row_factor[i] + col_factor[j];
            for (size_type k = 1; k < cols1; k += 2) {
                res->data[i][j] += (mtr1->data[i][k - 1] + mtr2->data[k    ][j]) *
                                   (mtr1->data[i][k    ] + mtr2->data[k - 1][j]);
            }
            if (flag) {
                res->data[i][j] += mtr1->data[i][cols1 - 1] * mtr2->data[cols1 - 1][j];
            }
        }
    }

    free(col_factor);
    free(row_factor);
    
    return res;

    free_row_factor:
    free(row_factor);
    end:
    free_matrix(res);
    return res;
}
