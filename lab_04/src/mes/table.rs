use std::fs::File;
use csv::{WriterBuilder, ReaderBuilder};

pub const STD_MES_FILENAME: &str = "mes.csv";

pub fn write_res_to_file(filename: &str, res_table: &[Vec<usize>], threads_cnt_vec: &[usize]) {
    let file = File::create(filename).unwrap();
    let mut wtr = WriterBuilder::new()
        .delimiter(b'|')
        .from_writer(file);

    wtr.write_record(&["Количество потоков", "Суммарное время работы(нс)"]).unwrap();
    for i in 0..threads_cnt_vec.len() {
        let s: usize = res_table[i].iter().sum();
        wtr.write_record(&[threads_cnt_vec[i].to_string(), s.to_string()]).unwrap();
    }

    wtr.flush().unwrap();
}

pub fn read_csv_for_tex(filename: &str, table_name: &str) {
    let file = File::open(filename).unwrap();
    let mut rdr = ReaderBuilder::new()
        .delimiter(b'|')
        .from_reader(file);

    let headers = rdr.headers().unwrap();
    let header1 = headers.get(0).unwrap_or("").to_string();
    let header2 = headers.get(1).unwrap_or("").to_string();

    println!(r"\begin{{table}}[h!]");
    println!("    \\begin{{center}}");
    println!("        \\begin{{threeparttable}}");
    println!("    \\caption{{Описание тестовых случаев}}");
    println!("    \\captionsetup{{justification=raggedright, singlelinecheck=false}}");
    println!("    \\label{{tbl:{table_name}}}");
    println!("    \\begin{{tabular}}{{|r|r|}}");
    println!("        \\hline");

    println!("        \\textbf{{{}}} & \\textbf{{{}}} \\\\", header1, header2);
    println!("        \\hline");

    for result in rdr.records() {
        let data = result.unwrap();
        let data1 = data.get(0).unwrap_or("").to_string();
        let data2 = data.get(1).unwrap_or("").to_string();

        println!("        {} & {} \\\\", data1, data2);
        println!("        \\hline");
    }

    println!("    \\end{{tabular}}");
    println!("    \\end{{threeparttable}}");
    println!("    \\end{{center}}");
    println!(r"\end{{table}}");
}
