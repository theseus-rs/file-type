use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_472896659: FileType = FileType {
    file_format: &FileFormat {
        id: 472_896_659,
        source_type: SourceType::Linguist,
        name: "FreeBASIC",
        extensions: &["bas", "bi"],
        media_types: &["text/x-vb"],
        signatures: &[],
        related_formats: &[],
    },
};
