use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_6: FileType = FileType {
    file_format: &FileFormat {
        id: 6,
        source_type: SourceType::Linguist,
        name: "APL",
        extensions: &["apl", "dyalog"],
        media_types: &["text/apl"],
        signatures: &[],
        related_formats: &[],
    },
};
