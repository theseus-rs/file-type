use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_687511714: FileType = FileType {
    file_format: &FileFormat {
        id: 687_511_714,
        source_type: SourceType::Linguist,
        name: "ShellCheck Config",
        extensions: &[],
        media_types: &["text/x-properties"],
        signatures: &[],
        related_formats: &[],
    },
};
