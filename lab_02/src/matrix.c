#include "matrix.h"

#include <stdlib.h>

// Выделение памяти ================================================================
value_type **alloc_matrix(const size_type rows, const size_type cols) {
    value_type **data = (value_type **)malloc(rows * sizeof(value_type *));
    if (!data) return data;

    for (size_type i = 0; i < rows; ++i) {
        data[i] = (value_type *)malloc(cols * sizeof(value_type));
        if (!data[i]) {
            free_data(data, i);
            return NULL;
        }
    }
    return data;
}

// Конструкторы ================================================================
matrix_t *init_matrix_by_size(const size_type rows, const size_type cols) {
    matrix_t *mtr = (matrix_t *)malloc(sizeof(matrix_t));
    if (!mtr) return NULL;

    mtr->rows = rows;
    mtr->cols = cols;
    mtr->data = alloc_matrix(rows, cols);
    if (!mtr->data) return NULL;

    for (size_type i = 0; i < rows; ++i) {
        for (size_type j = 0; j < cols; ++j) {
            mtr->data[i][j] = 0;
        }
    }

    return mtr;
}
matrix_t *init_matrix_by_array(const size_type rows, const size_type cols, data_t data) {
    matrix_t *mtr = (matrix_t *)malloc(sizeof(matrix_t));
    if (!mtr) return NULL;

    mtr->rows = rows;
    mtr->cols = cols;
    mtr->data = alloc_matrix(rows, cols);
    if (!mtr->data) return NULL;

    for (size_type i = 0; i < rows; ++i) {
        for (size_type j = 0; j < cols; ++j) {
            mtr->data[i][j] = data[i][j];
        }
    }

    return mtr;
}
matrix_t *init_matrix_by_size_with_rep_el(const size_type rows, const size_type cols, const value_type el) {
    matrix_t *mtr = init_matrix_by_size(rows, cols);
    if (!mtr) return NULL;

    for (size_type i = 0; i < rows; ++i) {
        for (size_type j = 0; j < cols; ++j) {
            mtr->data[i][j] = el;
        }
    }

    return mtr;
}
// IO ===============================================================================
#include <stdio.h>
void matrix_print(const matrix_t *mtr) {
    printf("Размеры матрицы: %zu %zu\n", mtr->rows, mtr->cols);
    printf("Матрица:\n");
    for (size_type i = 0; i < mtr->rows; ++i) {
        for (size_type j = 0; j < mtr->cols; ++j) {
            printf("%.2lf ", mtr->data[i][j]);
        }
        printf("\n");
    }
}

static int matrix_size(size_type *n, size_type *m) {
    int row, col = 0;
    printf("Введите размеры матрицы: ");
    if (scanf("%d%d", &row, &col) != 2) {
        return -1;
    }
    if (row <= 0 || col <= 0) {
        return -1;
    }
    *n = (size_type) row;
    *m = (size_type) col;
    return 0;
}
static int matrix_els(matrix_t *m) {
    printf("Ввод элементов матрицы\n------------\n");
    for (size_type i = 0; i < m->rows; i++) {
        for (size_type j = 0; j < m->cols; j++) {
            if (scanf("%lf", &m->data[i][j]) != 1) {
                return -1;
            }
        }
    }
    return 0;
}
matrix_t *matrix_input() {
    size_type n, m;
    int err = matrix_size(&n, &m);
    if (err) goto err_sizes_io;
    matrix_t *res = malloc(sizeof(matrix_t));
    if (!res) goto err_memory;

    res->rows = n;
    res->cols = m;
    res->data = alloc_matrix(n, m);
    if (!res->data) goto err_data_mem;

    err = matrix_els(res);
    if (err) goto err_mtrx_io;

    return res;

    err_mtrx_io:
    free_matrix(res);
    printf("ERROR: Incorrect matrix input\n");
    goto end;

    err_sizes_io:
    printf("ERROR: Incorrect sizes input\n");
    goto end;

    err_data_mem:
    free(res);
    err_memory:
    printf("ERROR: Can't allocate mamory");
    end:
    return NULL;
}

// Освобождениие памяти ================================================================
void free_matrix(matrix_t *matrix) {
    free_data(matrix->data, matrix->rows);
    free(matrix);
}
void free_data(value_type **data, const size_type rows) {
    for (size_type i = 0; i < rows; ++i) {
        free(data[i]);
    }
    free(data);
}
