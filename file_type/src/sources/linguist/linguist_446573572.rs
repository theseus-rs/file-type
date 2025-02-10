use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_446573572: FileType = FileType {
    file_format: &FileFormat {
        id: 446_573_572,
        source_type: SourceType::Linguist,
        name: "Nushell",
        extensions: &["nu"],
        media_types: &["text/x-sh"],
        signatures: &[],
        related_formats: &[],
    },
};
