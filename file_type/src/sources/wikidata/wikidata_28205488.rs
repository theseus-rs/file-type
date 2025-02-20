use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28205488: FileType = FileType {
    file_format: &FileFormat {
        id: 28_205_488,
        source_type: SourceType::Wikidata,
        name: "Windows Cursor",
        extensions: &["cur"],
        media_types: &["image/x-win-bitmap"],
        signatures: &[],
        related_formats: &[],
    },
};
