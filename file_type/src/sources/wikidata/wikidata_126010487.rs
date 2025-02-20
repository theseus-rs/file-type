use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_126010487: FileType = FileType {
    file_format: &FileFormat {
        id: 126_010_487,
        source_type: SourceType::Wikidata,
        name: "Sibelius Score 2020.3-2022.5",
        extensions: &["sib"],
        media_types: &["application/x-sibelius-score"],
        signatures: &[],
        related_formats: &[],
    },
};
