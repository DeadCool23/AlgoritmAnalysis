use super::cities::AncientWorld;

use std::{fs::File, io::{self, Write}};

fn matrix_to_tex(file: &mut File, matrix: &Vec<Vec<usize>>, data_class_num: usize) -> io::Result<()> {

    writeln!(file, "\\begin{{equation}}")?;
    writeln!(file, "\\label{{matrix:data_class_{}}}", data_class_num)?;
    writeln!(file, "M_{{{}}} = \\begin{{pmatrix}}", data_class_num)?;

    for (i, row) in matrix.iter().enumerate() {
        let row_str: String = row.iter().map(|&x| x.to_string()).collect::<Vec<_>>().join(" & ");
        if i < matrix.len() - 1 {
            writeln!(file, "    {} \\\\", row_str)?;
        } else {
            writeln!(file, "    {}", row_str)?;
        }
    }

    writeln!(file, "\\end{{pmatrix}}")?;
    writeln!(file, "\\end{{equation}}")?;

    Ok(())
}

fn etalon_marshrut_len_to_tex(file: &mut File, ancient_world: &AncientWorld, data_class_num: usize) -> io::Result<()> {
    writeln!(
        file, 
        "Эталонная длина маршрута для графа~\\ref{{matrix:data_class_{data_class_num}}} --- {} км.", 
        ancient_world.solve_tsp_by_brute_force().0
    )?;

    Ok(())
}

fn write_table_header(file: &mut impl Write, countries_cnt: usize) -> io::Result<()> {
    write!(file, "\\begin{{longtable}}{{|r|r|r|")?;
    for _ in 0..countries_cnt {
        write!(file, "r|r|r|")?;
    }
    writeln!(file, "}}")?;

    writeln!(
        file,
        "\\caption{{Результаты параметризации муравьиного алгоритма (начало)}}\\label{{tbl:param}}"
    )?;
    writeln!(file, "\\\\ \\hline")?;
    write!(file, "\\multicolumn{{3}}{{|c|}}{{Параметры}} ")?;
    for i in 0..countries_cnt {
        write!(file, "& \\multicolumn{{3}}{{|c|}}{{Граф {}}} ", i + 1)?;
    }
    writeln!(file, "\\\\")?;
    writeln!(file, "\\hline")?;
    write!(file, "$\\alpha$ & $\\rho$ & Дни ")?;
    for _ in 0..countries_cnt {
        write!(file, "& min & max & avg ")?;
    }
    writeln!(file, "\\\\")?;
    writeln!(file, "\\hline")?;

    writeln!(file, "\\endfirsthead")?;
    writeln!(file, "\\caption{{Результаты параметризации муравьиного алгоритма (продолжение)}}")?;
    writeln!(file, "\\\\ \\hline")?;
    write!(file, "\\multicolumn{{3}}{{|c|}}{{Параметры}} ")?;
    for i in 0..countries_cnt {
        write!(file, "& \\multicolumn{{3}}{{|c|}}{{Граф {}}} ", i + 1)?;
    }
    writeln!(file, "\\\\")?;
    writeln!(file, "\\hline")?;
    write!(file, "$\\alpha$ & $\\rho$ & Дни ")?;
    for _ in 0..countries_cnt {
        write!(file, "& min & max & avg ")?;
    }
    writeln!(file, "\\\\")?;
    writeln!(file, "\\hline")?;

    writeln!(file, "\\endhead")?;
    writeln!(file, "\\hline")?;
    writeln!(file, "\\endfoot")?;

    writeln!(file, "\\caption{{Результаты параметризации муравьиного алгоритма (окончание)}}")?;
    writeln!(file, "\\endlastfoot")?;
    writeln!(file, "\\hline")?;

    Ok(())
}

fn ant_alg_parametrization_to_tex(file: &mut File, countries: &[AncientWorld]) -> io::Result<()> {
    const REPS: usize = 5;
    const CONST_BETA: f64 = 0.75;

    write_table_header(file, countries.len())?;

    let coefs = vec![0.1, 0.25, 0.5, 0.75, 0.9];
    let days = Vec::<usize>::from([10, 20, 50, 100, 200]);

    let mut real_results = vec![];
    for ancient_world in countries.iter() {
        real_results.push(ancient_world.solve_tsp_by_brute_force().0);
    }

    for alpha in coefs.iter() {
        for rho in coefs.iter() {
            for day in days.iter() {
                write!(file, "{:.2} & {:.2} & {} ", alpha, rho, day)?;
                for (i, ancient_world) in countries.iter().enumerate() {
                    let mut results = vec![];
                    for _ in 0..=REPS {
                        results.push(ancient_world.solve_tsp_by_ant_colony(
                            *alpha, CONST_BETA, *rho, ancient_world.cities_count, *day, 0
                        ).0);
                    }
                    let devides: Vec<usize> = results
                        .iter()
                        .map(|&res| real_results[i].abs_diff(res))
                        .collect();

                    let min_devide = *devides.iter().min().unwrap() as f64;
                    let max_devide = *devides.iter().max().unwrap() as f64;
                    let avg_devide = devides.iter().sum::<usize>() as f64 / devides.len() as f64;
                    write!(file, "& {:.2} & {:.2} & {:.2} ", min_devide, max_devide, avg_devide)?;
                }
                writeln!(file, " \\\\")?;
            }
            writeln!(file, "\\hline")?;
        }
    }

    writeln!(file, "\\end{{longtable}}")?;

    Ok(())
}


pub fn auto_parametrization(countries: &[AncientWorld], tex_filename: &str) -> io::Result<()> {
    let mut file = File::create(tex_filename)?;

    for (i, ancient_world) in countries.iter().enumerate() {
        let data_class_num = i + 1;
        matrix_to_tex(&mut file, &ancient_world.cities_roads, data_class_num)?;
        writeln!(file, "")?;
        etalon_marshrut_len_to_tex(&mut file, ancient_world, data_class_num)?;
        writeln!(file, "")?;

    }

    ant_alg_parametrization_to_tex(&mut file, countries)?;

    Ok(())
}
