use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_125999326: FileType = FileType {
    file_format: &FileFormat {
        id: 125_999_326,
        source_type: SourceType::Wikidata,
        name: "Sibelius Score 7",
        extensions: &["sib"],
        media_types: &["application/x-sibelius-score"],
        signatures: &[],
        related_formats: &[],
    },
};
