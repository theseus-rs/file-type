use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_130633933: FileType = FileType {
    file_format: &FileFormat {
        id: 130_633_933,
        source_type: SourceType::Wikidata,
        name: "Ride source code file",
        extensions: &["ride"],
        media_types: &["text/x-ride"],
        signatures: &[],
        related_formats: &[],
    },
};
