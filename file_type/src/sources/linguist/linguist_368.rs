use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_368: FileType = FileType {
    file_format: &FileFormat {
        id: 368,
        source_type: SourceType::Linguist,
        name: "Tcsh",
        extensions: &["csh", "tcsh"],
        media_types: &["text/x-sh"],
        signatures: &[],
        related_formats: &[],
    },
};
