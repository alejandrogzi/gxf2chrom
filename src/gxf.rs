use crate::attr::Attribute;

#[derive(Debug, PartialEq)]
pub struct Record {
    pub chr: String,
    pub feat: String,
    pub start: u32,
    pub end: u32,
    pub strand: char,
    pub attr: Attribute,
}

impl Record {
    pub fn parse(line: &str, feat: &String) -> Result<Self, &'static str> {
        if line.is_empty() {
            return Err("Empty line");
        }

        let fields: Vec<&str> = line.split('\t').collect();

        if fields.len() < 9 {
            return Err("Line has fewer fields than expected".into());
        }

        let attr = Attribute::parse(&fields[8].to_string(), feat)
            .map_err(|_| "Error parsing attribute")?;

        Ok(Self {
            chr: String::from(fields[0]),
            feat: String::from(fields[2]),
            start: fields[3].parse::<u32>().unwrap() - 1,
            end: fields[4].parse().unwrap(),
            strand: fields[6].parse().unwrap(),
            attr: attr,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_record_gtf() {
        let line = "chr1\tunknown\texon\t11869\t12227\t.\t+\t.\tgene_id \"DDX11L1\"; gene_name \"DDX11L1\"; gene_source \"ensembl_havana\";
        gene_biotype \"transcribed_unprocessed_pseudogene\";".to_string();
        let feature = "gene_id".to_string();
        let record = Record::parse(&line, &feature).unwrap();
        assert_eq!(record.chr, "chr1");
        assert_eq!(record.feat, "exon");
        assert_eq!(record.start, 11868);
        assert_eq!(record.end, 12227);
        assert_eq!(record.strand, '+');
    }

    #[test]
    fn test_record_gff() {
        let line = "chr1\tunknown\texon\t11869\t12227\t.\t+\t.\tID=ENSG00000223972;Name=DDX11L1;biotype=transcribed_unprocessed_pseudogene";
        let feature = "ID".to_string();
        let record = Record::parse(&line, &feature).unwrap();
        assert_eq!(record.chr, "chr1");
        assert_eq!(record.feat, "exon");
        assert_eq!(record.start, 11868);
        assert_eq!(record.end, 12227);
        assert_eq!(record.strand, '+');
    }

    #[test]
    fn test_empty_line() {
        let line = "";
        let feature = "ID".to_string();
        let record = Record::parse(&line, &feature);
        assert_eq!(record, Err("Empty line"));
    }
}
