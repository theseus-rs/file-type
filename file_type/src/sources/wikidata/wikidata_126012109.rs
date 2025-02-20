use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_126012109: FileType = FileType {
    file_format: &FileFormat {
        id: 126_012_109,
        source_type: SourceType::Wikidata,
        name: "Sibelius Score 2022.12-2023.3",
        extensions: &["sib"],
        media_types: &["application/x-sibelius-score"],
        signatures: &[],
        related_formats: &[],
    },
};
