use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_902995658: FileType = FileType {
    file_format: &FileFormat {
        id: 902_995_658,
        source_type: SourceType::Linguist,
        name: "Genero per",
        extensions: &["per"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
