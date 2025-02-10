use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_126033191: FileType = FileType {
    file_format: &FileFormat {
        id: 126_033_191,
        source_type: SourceType::Wikidata,
        name: "Sibelius Score 2024",
        extensions: &["sib"],
        media_types: &["application/x-sibelius-score"],
        signatures: &[],
        related_formats: &[],
    },
};
