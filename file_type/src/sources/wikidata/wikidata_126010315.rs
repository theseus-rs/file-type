use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_126010315: FileType = FileType {
    file_format: &FileFormat {
        id: 126_010_315,
        source_type: SourceType::Wikidata,
        name: "Sibelius Score 2020.1",
        extensions: &["sib"],
        media_types: &["application/x-sibelius-score"],
        signatures: &[],
        related_formats: &[],
    },
};
