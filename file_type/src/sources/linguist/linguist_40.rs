use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_40: FileType = FileType {
    file_format: &FileFormat {
        id: 40,
        source_type: SourceType::Linguist,
        name: "Zeek",
        extensions: &["bro", "zeek"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
