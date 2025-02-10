use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_242: FileType = FileType {
    file_format: &FileFormat {
        id: 242,
        source_type: SourceType::Linguist,
        name: "NSIS",
        extensions: &["nsh", "nsi"],
        media_types: &["text/x-nsis"],
        signatures: &[],
        related_formats: &[],
    },
};
