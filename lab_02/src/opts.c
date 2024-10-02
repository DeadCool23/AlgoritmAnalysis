#include "opts.h"

#include <getopt.h>
#include <stdlib.h>

#include "mes.h"
#include "matrix.h"

options_t get_options(int argc, char **argv) {
    static struct option long_options[] = {
        {"interactive", no_argument,       0, 'i'},
        {"measure",     optional_argument, 0, 'm'},
        {"help",        no_argument,       0, 'h'},
        {0, 0, 0, 0}
    };

    int c;
    int option_index = 0;
    options_t opts = {.opt = NO_ARGS};

    while ((c = getopt_long(argc, argv, "im::h", long_options, &option_index)) != -1) {
        switch (c) {
            case 'i':
                opts.opt = INTERACTIVE;
                break;
            case 'm':
                opts.opt = MEASURES;
                if (optarg) {
                    opts.f = fopen(optarg, "w");
                } else {
                    opts.f = stdout;
                }
                break;
            case 'h':
                opts.opt = HELP;
                break;
            case '?':
                opts.opt = UNKNOWN;
                break;
            default:
                opts.opt = NO_ARGS;
                break;
        }
    }

    return opts;
}

void print_help() {
    printf("Использование: program [OPTIONS]\n");
    printf("Опции:\n");
    printf("  -i, --interactive       Запускает программу в интерактивном режиме\n");
    printf("  -m, --measure [file]    Сохраняет измерения в [file] или stdout если файл не задан\n");
    printf("  -h, --help              Показать эту справку и выйти\n");
}

void print_unknown() {
    printf("Для получения справки используйте опцию -h или --help.\n");
}

static int input_alg() {
    printf("Выберете алгоритм умножения матриц:\n");
    printf("Стандартный алгоритм умножения - s\n");
    printf("Алгоритм Винограда - w\n");
    printf("Оптимизированный алгоритм Винограда - o\n");
    printf(": ");

    char alg;
    if (scanf("%c", &alg) != 1) {
        return -1;
    }

    switch (alg) {
        case 's':
            return 0;
        case 'w':
            return 1;
        case 'o':
            return 2;
        default:
            return -1;
    }
}
int interactive_mult() {
    int alg_index = input_alg();
    if (alg_index == -1) {
        printf("ERROR: Incorrect algorithm choose\n");
        return -1;
    }

    matrix_alg_t mult = STD_ALGS[alg_index];

    printf("\nВведите матрицу №1:\n");
    matrix_t *m1 = matrix_input();
    if (!m1) goto end;

    printf("\nВведите матрицу №2:\n");
    matrix_t *m2 = matrix_input();
    if (!m2) goto clean_m1;

    if (m1->cols != m2->rows) goto err_sizes;

    matrix_t *res = mult(m1, m2);
    if (!res) goto clean_m2;

    printf("\n\nМатрица результат:\n");
    matrix_print(res);

    free_matrix(res);
    free_matrix(m2);
    free_matrix(m1);
    return 0;

    err_sizes:
    printf("ERROR: matrix1.rows != matrix2.cols\n");
    return -1;

    clean_m2:
    free_matrix(m2);
    clean_m1:
    free_matrix(m1);

    end:
    return -1;
}

int get_measures(FILE *f) {
    int err = 0;

    if ((err = print_measures(STD_TITLES, STD_ALGS, &STD_SIZES, f)) == -1) {
        printf("ERROR: can't get measurment\n");
    }
    if (f != stdout) { fclose(f); }
    return err;
}
