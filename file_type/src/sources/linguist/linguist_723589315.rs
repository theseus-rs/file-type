use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_723589315: FileType = FileType {
    file_format: &FileFormat {
        id: 723_589_315,
        source_type: SourceType::Linguist,
        name: "Option List",
        extensions: &[],
        media_types: &["text/x-sh"],
        signatures: &[],
        related_formats: &[],
    },
};
