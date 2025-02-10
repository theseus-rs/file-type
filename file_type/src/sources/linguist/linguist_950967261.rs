use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_950967261: FileType = FileType {
    file_format: &FileFormat {
        id: 950_967_261,
        source_type: SourceType::Linguist,
        name: "Win32 Message File",
        extensions: &["mc"],
        media_types: &["text/x-properties"],
        signatures: &[],
        related_formats: &[],
    },
};
