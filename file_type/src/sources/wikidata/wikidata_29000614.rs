use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_29000614: FileType = FileType {
    file_format: &FileFormat {
        id: 29_000_614,
        source_type: SourceType::Wikidata,
        name: "Resource File",
        extensions: &["res"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
