use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_125959218: FileType = FileType {
    file_format: &FileFormat {
        id: 125_959_218,
        source_type: SourceType::Wikidata,
        name: "Sibelius Score 2",
        extensions: &["sib"],
        media_types: &["application/x-sibelius-score"],
        signatures: &[],
        related_formats: &[],
    },
};
