use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_265: FileType = FileType {
    file_format: &FileFormat {
        id: 265,
        source_type: SourceType::Linguist,
        name: "OpenRC runscript",
        extensions: &[],
        media_types: &["text/x-sh"],
        signatures: &[],
        related_formats: &[],
    },
};
