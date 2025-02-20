use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_125998577: FileType = FileType {
    file_format: &FileFormat {
        id: 125_998_577,
        source_type: SourceType::Wikidata,
        name: "Sibelius Score 5",
        extensions: &["sib"],
        media_types: &["application/x-sibelius-score"],
        signatures: &[],
        related_formats: &[],
    },
};
