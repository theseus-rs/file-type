use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28344021: FileType = FileType {
    file_format: &FileFormat {
        id: 28_344_021,
        source_type: SourceType::Wikidata,
        name: "Imagine Object File",
        extensions: &["iob", "obj"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
