use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_125999013: FileType = FileType {
    file_format: &FileFormat {
        id: 125_999_013,
        source_type: SourceType::Wikidata,
        name: "Sibelius Score 6",
        extensions: &["sib"],
        media_types: &["application/x-sibelius-score"],
        signatures: &[],
        related_formats: &[],
    },
};
