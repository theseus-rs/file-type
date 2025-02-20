use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_125997892: FileType = FileType {
    file_format: &FileFormat {
        id: 125_997_892,
        source_type: SourceType::Wikidata,
        name: "Sibelius Score 3",
        extensions: &["sib"],
        media_types: &["application/x-sibelius-score"],
        signatures: &[],
        related_formats: &[],
    },
};
