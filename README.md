![Crates.io](https://img.shields.io/crates/v/gxf2chrom?color=green)
![GitHub](https://img.shields.io/github/license/alejandrogzi/gxf2chrom?color=blue)
![Crates.io Total Downloads](https://img.shields.io/crates/d/gxf2chrom)
![Conda Platform](https://img.shields.io/conda/pn/bioconda/gxf2chrom)

# gxf2chrom
Everything in .chrom from GTF/GFF files.

turns GTF/GFF files into:
```
ENSP00000501388.1       chr1    -       19417068        19485438
ENSP00000438792.1       chr11   -       72754899        72781181
ENSP00000224756.8       chr10   +       84371052        84513540
ENSP00000415935.1       chr2    +       215312481       215349652
```

Some stast on different species:
- *Homo sapiens* GRCh38 GENCODE 44 (252,835 transcripts) in 1.84 seconds.
- *Mus musculus* GRCm39 GENCODE 44 (149,547 transcritps) in 1.02 seconds.
- *Canis lupus familiaris* ROS_Cfam_1.0 Ensembl 110 (55,335 transcripts) in 0.57 seconds. 
- *Gallus galus* bGalGal1 Ensembl 110 (72,689 transcripts) in 0.62 seconds.


## Usage
``` rust
Usage: gxf2chrom [OPTIONS] --input <GXF> --output <CHROM>
 
Arguments:
    --input/-i <GXF>: a .gtf/gff/*.gz file
    --output/-o <CHROM>: path to output .chrom file

Options:
    --help/-h: print help
    --version/-v: print version
    --feature/-f: feature to extract [default: protein_id]
    --threads/-t: number of threads [default: max ncpus]
```

## Installation
to install gxf2chrom on your system follow this steps:
1. get rust: `curl https://sh.rustup.rs -sSf | sh` on unix, or go [here](https://www.rust-lang.org/tools/install) for other options
2. run `cargo install gxf2chrom` (make sure `~/.cargo/bin` is in your `$PATH` before running it)
4. use `gxf2chrom` with the required arguments
5. enjoy!

## Build
to build gxf2chrom from this repo, do:

1. get rust (as described above)
2. run `git clone https://github.com/alejandrogzi/gxf2chrom.git && cd gxf2chrom`
3. run `cargo run --release -- -i <GXF> -o <CHROM> [OPTIONS]`

## Container image
to build the development container image:
1. run `git clone https://github.com/alejandrogzi/gxf2chrom.git && cd gxf2chrom`
2. initialize docker with `start docker` or `systemctl start docker`
3. build the image `docker image build --tag gxf2chrom .`
4. run `docker run --rm -v "[dir_where_your_gxf_is]:/dir" gxf2chrom -i /dir/<GXF> -o /dir/<CHROM>`

## Conda
to use gxf2chrom through Conda just:
1. `conda install gxf2chrom -c bioconda` or `conda create -n gxf2chrom -c bioconda gtfsort`

## Benchmark

This tool was inspired by [NCBIgff2chrom.py](https://github.com/conchoecia/odp/blob/main/docs/scripts/NCBIgff2chrom.py) from the [Oxford dot plots](https://github.com/conchoecia/odp) repository. to quickly convert GTF/GFF files into .chrom files. Here is a quick benchmark:


<div align="center">

| Format  | odp      | gxf2chrom | fold |
|:---------:|:--------------------:|:--------------------:|:--------------------:|
| gff3    | 4.30 +/- 0.03      | 1.88 +/- 0.01      | x2.29 |
| gff3.gz | 6.27 +/- 0.18      | 2.05 +/- 0.01      | x3.06 |
| gtf     | ---      | 1.83 +/- 0.01      | --- |
| gtf.gz  | ---      | 1.94 +/- 0.01      | --- |

</div>

With faster times, gxf2chrom intends to be a versatile approach to be implemented in any pipeline!
