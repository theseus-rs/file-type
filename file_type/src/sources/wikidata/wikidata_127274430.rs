use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_127274430: FileType = FileType {
    file_format: &FileFormat {
        id: 127_274_430,
        source_type: SourceType::Wikidata,
        name: "NX Elysium Neutral File",
        extensions: &["enf_abq"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
