use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_61886938: FileType = FileType {
    file_format: &FileFormat {
        id: 61_886_938,
        source_type: SourceType::Wikidata,
        name: "Portable Form File",
        extensions: &["pff"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
