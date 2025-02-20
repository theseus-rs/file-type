use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_125998297: FileType = FileType {
    file_format: &FileFormat {
        id: 125_998_297,
        source_type: SourceType::Wikidata,
        name: "Sibelius Score 4",
        extensions: &["sib"],
        media_types: &["application/x-sibelius-score"],
        signatures: &[],
        related_formats: &[],
    },
};
