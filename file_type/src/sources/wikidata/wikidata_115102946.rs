use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_115102946: FileType = FileType {
    file_format: &FileFormat {
        id: 115_102_946,
        source_type: SourceType::Wikidata,
        name: "BFRES file",
        extensions: &["bfres"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
