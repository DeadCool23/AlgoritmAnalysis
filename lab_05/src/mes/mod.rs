
pub mod plot;

use std::cmp::min;
use std::fs::File;
use std::collections::HashMap;

use csv::WriterBuilder;

pub const STD_MES_FILENAME: &str = "mes.csv";
pub const STD_TIMEDATA_SIZES: (usize, usize, usize) = (3, 2, 2);

#[derive(Debug, Clone)]
pub struct TimeData {
    reader_times: Vec<(f64, f64)>,
    parser_times: Vec<(f64, f64)>,
    writer_times: Vec<(f64, f64)>,
}

#[allow(dead_code)]
impl TimeData {
    pub fn new(reader_times: &[(f64, f64)], parser_times: &[(f64, f64)], writer_times: &[(f64, f64)]) -> Self {
        TimeData {
            reader_times: Vec::from(reader_times),
            parser_times: Vec::from(parser_times),
            writer_times: Vec::from(writer_times)
        }
    }

    pub fn cut(&self, reader_times_cnt: usize, parser_times_cnt: usize, writer_times_cnt: usize) -> Option<TimeData> {
        if reader_times_cnt > self.reader_times.len() || parser_times_cnt > self.parser_times.len() || writer_times_cnt > self.writer_times.len() {
            return None;
        }

        Some(TimeData {
            reader_times: self.reader_times.iter().take(reader_times_cnt).cloned().collect(),
            parser_times: self.parser_times.iter().take(parser_times_cnt).cloned().collect(),
            writer_times: self.writer_times.iter().take(writer_times_cnt).cloned().collect()
        })
    }

    pub fn get_avg_processing_times(&self) -> HashMap<String, f64> {
        let mut avg_times = HashMap::new();
    
        let reader_avg = self.reader_times.iter()
            .map(|(start, end)| end - start)
            .sum::<f64>() / self.reader_times.len() as f64;
        avg_times.insert("reader".to_string(), reader_avg);
    
        let parser_avg = self.parser_times.iter()
            .map(|(start, end)| end - start)
            .sum::<f64>() / self.parser_times.len() as f64;
        avg_times.insert("parser".to_string(), parser_avg);
    
        let writer_avg = self.writer_times.iter()
            .map(|(start, end)| end - start)
            .sum::<f64>() / self.writer_times.len() as f64;
        avg_times.insert("writer".to_string(), writer_avg);
    
        avg_times
    }

    pub fn get_avg_waiting_times(&self) -> HashMap<String, f64> {
        let mut avg_times = HashMap::new();
    
        let reader_waiting_avg = if self.reader_times.len() > 1 {
            self.reader_times.windows(2)
                .map(|window| window[1].0 - window[0].1) // разница между началом следующего и концом текущего
                .sum::<f64>() / (self.reader_times.len() - 1) as f64
        } else {
            0.0
        };
        avg_times.insert("reader".to_string(), reader_waiting_avg);
    
        let parser_waiting_avg = if self.parser_times.len() > 1 {
            self.parser_times.windows(2)
                .map(|window| window[1].0 - window[0].1)
                .sum::<f64>() / (self.parser_times.len() - 1) as f64
        } else {
            0.0
        };
        avg_times.insert("parser".to_string(), parser_waiting_avg);
    
        let writer_waiting_avg = if self.writer_times.len() > 1 {
            self.writer_times.windows(2)
                .map(|window| window[1].0 - window[0].1)
                .sum::<f64>() / (self.writer_times.len() - 1) as f64
        } else {
            0.0
        };
        avg_times.insert("writer".to_string(), writer_waiting_avg);
    
        avg_times
    }    

    pub fn get_process_avg_time(&self) -> f64 {
        let cnt = min(min(self.reader_times.len(), self.parser_times.len()), self.writer_times.len());
        
        let total_time: f64 = (0..cnt)
            .map(|i| {
                (self.reader_times[i].1 - self.reader_times[i].0) +
                (self.parser_times[i].1 - self.parser_times[i].0) +
                (self.writer_times[i].1 - self.writer_times[i].0)
            })
            .sum();
        
        match cnt {
            0 => 0.,
            _ => total_time / cnt as f64
        }
    }
}

pub fn write_timedata_to_csv(filename: &str, timedata: &TimeData) {
    let file = File::create(filename).unwrap();
    let mut wtr = WriterBuilder::new()
        .delimiter(b'|')
        .from_writer(file);

    let threads_names = vec!["reader", "parser", "writer"];

    let proc_map = timedata.get_avg_processing_times();
    let wait_map = timedata.get_avg_waiting_times();

    wtr.write_record(&["Название потока", "Среднее время обработки", "Среднее время ожидания"]).unwrap();
    for name in threads_names {
        wtr.write_record(&[name, &format!("{:.4}", proc_map[name]), &format!("{:.4}", wait_map[name])]).unwrap();
    }

    wtr.flush().unwrap();
}
