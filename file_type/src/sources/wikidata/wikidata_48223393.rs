use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_48223393: FileType = FileType {
    file_format: &FileFormat {
        id: 48_223_393,
        source_type: SourceType::Wikidata,
        name: "PageMaker Time Stamp File",
        extensions: &["tym"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
