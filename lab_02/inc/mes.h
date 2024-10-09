#ifndef __MES_H__
#define __MES_H__

#include <stdio.h>

#include "matrix.h"

typedef struct {
    size_type start_mes;
    size_type end_mes;
    size_type step_mes;
} mes_size_t;

#define ALGS_CNT 3
#define MES_CNT 100

static char *STD_TITLES[ALGS_CNT] = {
    "std_mult",
    "winograd_mult",
    "opt_winograd_mult"
};

static matrix_alg_t STD_ALGS[ALGS_CNT] = {
    std_matrix_mult,
    winograd_matrix_mult,
    optimized_winograd_matrix_mult
};

static mes_size_t STD_SIZES = {
    .start_mes = 0,
    .end_mes = 50,
    .step_mes = 5
};

int print_measures(char *titles[ALGS_CNT] , matrix_alg_t algs[ALGS_CNT], mes_size_t *mes_sizes, FILE *f);

#endif // __MES_H__