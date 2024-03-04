use clap::{self, Parser};
use num_cpus;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Parser, Debug)]
#[clap(
    name = "gxf2chrom",
    version = "0.1.0",
    author = "Alejandro Gonzales-Irribarren <jose.gonzalesdezavala1@unmsm.edu.pe>",
    about = "Everythin in .chrom from GTF/GFF"
)]
pub struct Args {
    /// ...
    ///
    /// This program converts GTF/GFF3 files to .chrom format.
    /// Start by providing the path to the GTF/GFF3 file with -i/--input file.gtf
    /// or -i/--input file.gff3.
    #[clap(
        short = 'i',
        long = "input",
        help = "Path to GTF/GFF file",
        value_name = "GXF",
        required = true
    )]
    pub gxf: PathBuf,

    /// Output filepath; non-required argument.
    ///
    #[clap(
        short = 'o',
        long = "output",
        help = "Path to output .chrom file",
        value_name = "CHROM",
        required = true
    )]
    pub output: PathBuf,

    /// Number of threads to use; default is the number of logical CPUs.
    #[clap(
        short = 't',
        long,
        help = "Number of threads",
        value_name = "THREADS",
        default_value_t = num_cpus::get()
    )]
    pub threads: usize,

    /// Feature to extract; default is "protein_id".
    #[clap(
        short = 'f',
        long = "feature",
        help = "Feature to extract",
        value_name = "FEATURE",
        default_value = "protein_id"
    )]
    pub feature: String,
}

impl Args {
    /// Checks all the arguments for validity using validate_args()
    pub fn check(&self) -> Result<(), ArgError> {
        self.validate_args()
    }

    /// Checks the input file for validity. The file must exist and be a GTF or GFF3 file.
    /// If the file does not exist, an error is returned.
    fn check_input(&self) -> Result<(), ArgError> {
        if !self.gxf.exists() {
            let err = format!("file {:?} does not exist", self.gxf);
            return Err(ArgError::InvalidInput(err));
        } else if !self.gxf.extension().unwrap().eq("gff")
            & !self.gxf.extension().unwrap().eq("gtf")
            & !self.gxf.extension().unwrap().eq("gff3")
            & !self.gxf.extension().unwrap().eq("gz")
        {
            let err = format!(
                "file {:?} is not a GTF or GFF3 file, please specify the correct format",
                self.gxf
            );
            return Err(ArgError::InvalidInput(err));
        } else if std::fs::metadata(&self.gxf).unwrap().len() == 0 {
            let err = format!("file {:?} is empty", self.gxf);
            return Err(ArgError::InvalidInput(err));
        } else {
            Ok(())
        }
    }

    /// Checks the output file for validity. If the file is not a BED file, an error is returned.
    fn check_output(&self) -> Result<(), ArgError> {
        if !self.output.extension().unwrap().eq("chrom") {
            let err = format!("file {:?} is not a .chrom file", self.output);
            return Err(ArgError::InvalidOutput(err));
        } else {
            Ok(())
        }
    }

    /// Checks the number of threads for validity. The number of threads must be greater than 0
    /// and less than or equal to the number of logical CPUs.
    fn check_threads(&self) -> Result<(), ArgError> {
        if self.threads == 0 {
            let err = format!("number of threads must be greater than 0");
            return Err(ArgError::InvalidThreads(err));
        } else if self.threads > num_cpus::get() {
            let err = format!(
                "number of threads must be less than or equal to the number of logical CPUs"
            );
            return Err(ArgError::InvalidThreads(err));
        } else {
            Ok(())
        }
    }

    fn check_feature(&self) -> Result<(), ArgError> {
        if self.feature.is_empty() {
            let err = format!("feature must not be empty");
            return Err(ArgError::InvalidFeature(err));
        } else {
            Ok(())
        }
    }

    /// Validates all the arguments
    fn validate_args(&self) -> Result<(), ArgError> {
        self.check_input()?;
        self.check_output()?;
        self.check_threads()?;
        self.check_feature()?;
        Ok(())
    }

    pub fn get() -> Self {
        Args::parse()
    }
}

#[derive(Debug, Error)]
pub enum ArgError {
    /// The input file does not exist or is not a GTF or GFF3 file.
    #[error("Invalid input: {0}")]
    InvalidInput(String),

    /// The output file is not a BED file.
    #[error("Invalid output: {0}")]
    InvalidOutput(String),

    /// The number of threads is invalid.
    #[error("Invalid number of threads: {0}")]
    InvalidThreads(String),

    /// The feature is invalid.
    #[error("Invalid feature: {0}")]
    InvalidFeature(String),
}
