use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_7391883: FileType = FileType {
    file_format: &FileFormat {
        id: 7_391_883,
        source_type: SourceType::Wikidata,
        name: "SNP file format",
        extensions: &["snp"],
        media_types: &["application/vnd.ms-access", "image/x-snp"],
        signatures: &[],
        related_formats: &[],
    },
};
