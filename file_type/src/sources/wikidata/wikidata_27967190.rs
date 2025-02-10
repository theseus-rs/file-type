use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27967190: FileType = FileType {
    file_format: &FileFormat {
        id: 27_967_190,
        source_type: SourceType::Wikidata,
        name: "General Digital Music module",
        extensions: &["gdm"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
