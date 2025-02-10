use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_41: FileType = FileType {
    file_format: &FileFormat {
        id: 41,
        source_type: SourceType::Linguist,
        name: "C",
        extensions: &["c", "cats", "h", "h.in", "idc"],
        media_types: &["text/x-csrc"],
        signatures: &[],
        related_formats: &[],
    },
};
