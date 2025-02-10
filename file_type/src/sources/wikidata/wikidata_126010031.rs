use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_126010031: FileType = FileType {
    file_format: &FileFormat {
        id: 126_010_031,
        source_type: SourceType::Wikidata,
        name: "Sibelius Score 8.6-2019.12",
        extensions: &["sib"],
        media_types: &["application/x-sibelius-score"],
        signatures: &[],
        related_formats: &[],
    },
};
