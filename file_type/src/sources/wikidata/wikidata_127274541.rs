use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_127274541: FileType = FileType {
    file_format: &FileFormat {
        id: 127_274_541,
        source_type: SourceType::Wikidata,
        name: "Pro/ENGINEER Elysium Neutral File",
        extensions: &["enf_abq"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
