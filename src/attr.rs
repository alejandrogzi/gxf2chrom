#![allow(dead_code)]

use hashbrown::HashMap;
use thiserror::Error;

#[derive(Debug, PartialEq)]
pub struct Attribute {
    pub protein_id: String,
}

impl Attribute {
    pub fn parse(line: &String, feat: &String) -> Result<Attribute, ParseError> {
        if !line.is_empty() {
            let mut attributes: HashMap<String, String> = HashMap::new();
            let bytes = line.trim_end().as_bytes().iter().enumerate();

            let mut start = 0;

            for (mut i, byte) in bytes {
                if *byte == b';' || i == line.len() - 1 {
                    if i == line.len() - 1 && *byte != b';' {
                        i += 1;
                    };
                    let word = &line[start..i];
                    if !word.is_empty() {
                        let (key, value) = get_pair(word)?;
                        attributes.insert(key, value);
                    }
                    start = i + 1;
                }
            }

            let protein_id = attributes
                .get(feat)
                .ok_or(ParseError::MissingProtId(line.to_string()))?;

            Ok(Attribute {
                protein_id: protein_id.to_string(),
            })
        } else {
            Err(ParseError::Empty)
        }
    }
}

fn get_pair(line: &str) -> Result<(String, String), ParseError> {
    let line = line.trim();
    let mut bytes = line.as_bytes().iter();
    let i = bytes
        .position(|b| *b == b' ' || *b == b'=')
        .ok_or(ParseError::InvalidPair(line.to_string()))?;

    let key = &line[..i];
    let value = &line[i + 1..line.len()].trim_matches('"').trim();

    Ok((key.to_string(), value.to_string()))
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ParseError {
    // Empty line
    #[error("Empty line, cannot parse attributes")]
    Empty,

    // Invalid GTF line (unused for now)
    #[error("Invalid GXF line: {0}")]
    Invalid(String),

    // Invalid attribute pair, allow get_pair panic
    #[error("Invalid attribute pair: {0}")]
    InvalidPair(String),

    // Missing gene_id attribute
    #[error("Missing gene_id attribute in: {0}")]
    MissingProtId(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_attributes() {
        let input = "gene_id \"ABC\"; transcript_id \"XYZ\"; exon_number \"1\"; exon_id \"123\"; protein_id \"DEF\";"
            .to_string();
        let feat = "protein_id".to_string();
        let attr = Attribute::parse(&input, &feat).unwrap();

        assert_eq!(attr.protein_id, "DEF");
    }

    #[test]
    fn invalid_attributes() {
        let input = "transcript_id \"XYZ\"; exon_number \"1\";".to_string();
        let feat = "protein_id".to_string();
        let result = Attribute::parse(&input, &feat);

        assert_eq!(result.unwrap_err(), ParseError::MissingProtId(input));
    }

    #[test]
    fn get_gencode_pair_from_gene_line() {
        let line = "gene_id \"ENSG00000290825.1\"; gene_type \"lncRNA\"; gene_name \"DDX11L2\"; level 2; tag \"overlaps_pseudogene\";".to_string();
        let mut attributes: HashMap<String, String> = HashMap::new();
        let bytes = line.as_bytes().iter().enumerate();

        let mut start = 0;

        for (i, byte) in bytes {
            if *byte == b';' {
                let word = &line[start..i];
                if !word.is_empty() {
                    let (key, value) = get_pair(word).unwrap();
                    attributes.insert(key, value);
                }
                start = i + 2;
            }
        }

        assert_eq!(
            *attributes.get("gene_id").unwrap(),
            String::from("ENSG00000290825.1")
        );
        assert_eq!(
            *attributes.get("gene_type").unwrap(),
            String::from("lncRNA")
        );
        assert_eq!(
            *attributes.get("gene_name").unwrap(),
            String::from("DDX11L2")
        );
        assert_eq!(*attributes.get("level").unwrap(), String::from("2"));
        assert_eq!(
            *attributes.get("tag").unwrap(),
            String::from("overlaps_pseudogene")
        );
    }

    #[test]
    fn get_gencode_pair_from_exon_line() {
        let line = "gene_id \"ENSG00000290825.1\"; transcript_id \"ENST00000456328.2\"; gene_type \"lncRNA\"; gene_name \"DDX11L2\"; transcript_type \"lncRNA\"; transcript_name \"DDX11L2-202\"; exon_number 2; exon_id \"ENSE00003582793.1\"; level 2; transcript_support_level \"1\"; tag \"basic\"; tag \"Ensembl_canonical\"; havana_transcript \"OTTHUMT00000362751.1\";".to_string();
        let mut attributes: HashMap<String, String> = HashMap::new();

        let bytes = line.trim_end().as_bytes().iter().enumerate();

        let mut start = 0;

        for (mut i, byte) in bytes {
            if *byte == b';' || i == line.len() - 1 {
                if i == line.len() - 1 && *byte != b';' {
                    i += 1;
                };
                let word = &line[start..i];
                if !word.is_empty() {
                    let (key, value) = get_pair(word).unwrap();
                    attributes.insert(key, value);
                }
                start = i + 1;
            }
        }

        assert_eq!(
            *attributes.get("gene_id").unwrap(),
            String::from("ENSG00000290825.1")
        );
        assert_eq!(
            *attributes.get("transcript_id").unwrap(),
            String::from("ENST00000456328.2")
        );
        assert_eq!(
            *attributes.get("gene_type").unwrap(),
            String::from("lncRNA")
        );
        assert_eq!(
            *attributes.get("gene_name").unwrap(),
            String::from("DDX11L2")
        );
        assert_eq!(
            *attributes.get("transcript_type").unwrap(),
            String::from("lncRNA")
        );
        assert_eq!(
            *attributes.get("transcript_name").unwrap(),
            String::from("DDX11L2-202")
        );
        assert_eq!(*attributes.get("exon_number").unwrap(), String::from("2"));
        assert_eq!(
            *attributes.get("exon_id").unwrap(),
            String::from("ENSE00003582793.1")
        );
        assert_eq!(*attributes.get("level").unwrap(), String::from("2"));
        assert_eq!(
            *attributes.get("transcript_support_level").unwrap(),
            String::from("1")
        );
        assert_eq!(
            *attributes.get("tag").unwrap(),
            String::from("Ensembl_canonical")
        );
        assert_eq!(
            *attributes.get("havana_transcript").unwrap(),
            String::from("OTTHUMT00000362751.1")
        );
    }

    #[test]
    fn parse_gff_line() {
        let line = "chr1\tHAVANA\ttranscript\t11869\t14409\t.\t+\t.\tID=ENST00000450305.2;Parent=ENSG00000223972.6;gene_id=ENSG00000223972.6;transcript_id=ENST00000450305.2;gene_type=transcribed_unprocessed_pseudogene;gene_name=DDX11L1;transcript_type=transcribed_unprocessed_pseudogene;transcript_name=DDX11L1-201;level=2;transcript_support_level=NA;hgnc_id=HGNC:37102;ont=PGO:0000005,PGO:0000019;tag=basic,Ensembl_canonical;havana_gene=OTTHUMG00000000961.2;havana_transcript=OTTHUMT00000002844.2".to_string();
        let feat = "protein_id".to_string();
        let attr = Attribute::parse(&line, &feat);

        assert_eq!(attr.unwrap_err(), ParseError::MissingProtId(line));
    }
}
