use flate2::read::MultiGzDecoder;
use hashbrown::HashMap;
use rayon::prelude::*;
use std::fmt::Debug;
use std::fs::File;
use std::io::{self, BufWriter, Read, Write};
use std::path::Path;

use crate::gxf::Record;

pub type ProtRecord = HashMap<String, InnerProtRecord>;

#[derive(Debug, PartialEq)]
pub struct InnerProtRecord {
    pub chr: String,
    pub strand: char,
    pub start: u32,
    pub end: u32,
}

impl Default for InnerProtRecord {
    fn default() -> Self {
        InnerProtRecord {
            chr: String::new(),
            strand: ' ',
            start: 0,
            end: 0,
        }
    }
}

pub fn reader<P: AsRef<Path> + Debug>(f: P) -> io::Result<String> {
    let mut file = File::open(f.as_ref())?;

    if f.as_ref().extension().unwrap() == "gz" {
        let mut gz = MultiGzDecoder::new(file);
        let mut contents = String::new();
        gz.read_to_string(&mut contents)?;
        Ok(contents)
    } else {
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(contents)
    }
}

pub fn write_obj<P: AsRef<Path> + Debug>(file: P, obj: &ProtRecord) -> io::Result<()> {
    let file = File::create(file)?;
    let mut f = BufWriter::new(file);

    for (prot, record) in obj {
        writeln!(
            f,
            "{}\t{}\t{}\t{}\t{}",
            prot, record.chr, record.strand, record.start, record.end
        )?;
    }

    f.flush()?;
    Ok(())
}

pub fn parallel_parse<'a>(s: &'a str, f: &'a String) -> Result<ProtRecord, &'static str> {
    let x = s
        .par_lines()
        .filter(|line| !line.starts_with("#"))
        .filter_map(|line| Record::parse(line, f).ok())
        .filter(|record| record.feat == "CDS" && record.start < record.end)
        .fold(
            || HashMap::new(),
            |mut acc: ProtRecord, record| {
                // acc.entry(record.chr.clone()).or_default().push(record);
                let k = acc
                    .entry(record.attr.protein_id)
                    .or_insert_with(|| InnerProtRecord {
                        chr: record.chr,
                        strand: record.strand,
                        start: record.start,
                        end: record.end,
                    });

                if !k.chr.is_empty() {
                    k.start = k.start.min(record.start);
                    k.end = k.end.max(record.end);
                };

                acc
            },
        )
        .reduce(
            || HashMap::new(),
            |mut acc, map| {
                for (k, v) in map {
                    let x = acc.entry(k).or_default();
                    if !x.chr.is_empty() {
                        x.start = x.start.min(v.start);
                        x.end = x.end.max(v.end);
                    } else {
                        *x = v;
                    }
                }

                acc
            },
        );

    Ok(x)
}

pub fn max_mem_usage_mb() -> f64 {
    let rusage = unsafe {
        let mut rusage = std::mem::MaybeUninit::uninit();
        libc::getrusage(libc::RUSAGE_SELF, rusage.as_mut_ptr());
        rusage.assume_init()
    };
    let maxrss = rusage.ru_maxrss as f64;
    if cfg!(target_os = "macos") {
        maxrss / 1024.0 / 1024.0
    } else {
        maxrss / 1024.0
    }
}
