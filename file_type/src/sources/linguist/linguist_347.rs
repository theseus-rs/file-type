use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_347: FileType = FileType {
    file_format: &FileFormat {
        id: 347,
        source_type: SourceType::Linguist,
        name: "ShellSession",
        extensions: &["sh-session"],
        media_types: &["text/x-sh"],
        signatures: &[],
        related_formats: &[],
    },
};
