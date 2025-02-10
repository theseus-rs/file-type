use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_127268401: FileType = FileType {
    file_format: &FileFormat {
        id: 127_268_401,
        source_type: SourceType::Wikidata,
        name: "Elysium Neutral File",
        extensions: &["enf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
