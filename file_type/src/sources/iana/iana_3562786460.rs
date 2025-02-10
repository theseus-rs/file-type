use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3562786460: FileType = FileType {
    file_format: &FileFormat {
        id: 3_562_786_460,
        source_type: SourceType::Iana,
        name: "H224",
        extensions: &[],
        media_types: &["application/H224"],
        signatures: &[],
        related_formats: &[],
    },
};
