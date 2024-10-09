#include "mes.h"

#include <time.h>
#include <stdlib.h>

static size_type sizes_count(mes_size_t *mes_sizes) {
    return ((mes_sizes->end_mes - mes_sizes->start_mes) / mes_sizes->step_mes) + 1; 
}

double get_matrix_alg_mes(matrix_alg_t alg, size_type rows, size_type cols) {
    matrix_t *mtr1 = init_matrix_by_size_with_rep_el(rows, cols, 1);
    if (!mtr1) goto end;
    matrix_t *mtr2 = init_matrix_by_size_with_rep_el(rows, cols, 1);
    if (!mtr2) goto free_mtr1;

    double res_sum = 0;
    for (size_type i = 0; i < MES_CNT; ++i) {
        clock_t start = clock();
        matrix_t *res = alg(mtr1, mtr2);
        clock_t end = clock();

        if (res) free_matrix(res);
        res_sum += (1e3 * (end - start)) / CLOCKS_PER_SEC;
    }

    free_matrix(mtr1);
    free_matrix(mtr2);
    return res_sum / MES_CNT;

    free_mtr1: free_matrix(mtr1);
    end: return -1;
}
static double *get_matrixs_alg_mes(matrix_alg_t alg, mes_size_t *mes_sizes) {
    double *res = malloc(sizes_count(mes_sizes) * sizeof(double));
    if (!res) return NULL;

    size_t i = 0;
    for (size_type sizes = mes_sizes->start_mes; sizes <= mes_sizes->end_mes; sizes += mes_sizes->step_mes) {
        res[i++] = get_matrix_alg_mes(alg, sizes, sizes);
    }
    return res;
}
static double **get_matrixs_algs_mes(matrix_alg_t algs[ALGS_CNT], mes_size_t *mes_sizes) {
    double **res = malloc(ALGS_CNT * sizeof(double*));
    if (!res) return NULL;

    for (size_t i = 0; i < ALGS_CNT; ++i) {
        res[i] = get_matrixs_alg_mes(algs[i], mes_sizes);
        if (!res[i]) {
            free_data(res, i);
            return NULL;
        }
    }

    return res;
}

static void print_title(char *titles[ALGS_CNT], FILE *f) {
    fprintf(f, "matrix_size|");
    for (size_t i = 0; i < ALGS_CNT; ++i) {
        fprintf(f, !i ? "%s" : "|%s", titles[i]);
    }
    fprintf(f, "\n");
}
int print_measures(char *titles[ALGS_CNT] , matrix_alg_t algs[ALGS_CNT], mes_size_t *mes_sizes, FILE *f) {
    double **measures = get_matrixs_algs_mes(algs, mes_sizes);
    if (!measures) return -1;

    print_title(titles, f);
    
    size_t i = 0;
    for (size_type sizes = mes_sizes->start_mes; sizes <= mes_sizes->end_mes; sizes += mes_sizes->step_mes) {
        fprintf(f, "%zu|", sizes);
        for (size_t j = 0; j < ALGS_CNT; ++j) {
            fprintf(f, !j ? "%lf" : "|%lf", measures[j][i]);
        }
        fprintf(f, "\n");
        ++i;
    }
    
    free_data(measures, ALGS_CNT);
    return 0;
}
