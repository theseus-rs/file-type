use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_125999747: FileType = FileType {
    file_format: &FileFormat {
        id: 125_999_747,
        source_type: SourceType::Wikidata,
        name: "Sibelius Score 7.5-8.0",
        extensions: &["sib"],
        media_types: &["application/x-sibelius-score"],
        signatures: &[],
        related_formats: &[],
    },
};
