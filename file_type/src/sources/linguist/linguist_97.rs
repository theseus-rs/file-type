use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_97: FileType = FileType {
    file_format: &FileFormat {
        id: 97,
        source_type: SourceType::Linguist,
        name: "Eagle",
        extensions: &["brd", "sch"],
        media_types: &["text/xml"],
        signatures: &[],
        related_formats: &[],
    },
};
