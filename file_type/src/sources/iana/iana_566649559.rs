use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_566649559: FileType = FileType {
    file_format: &FileFormat {
        id: 566_649_559,
        source_type: SourceType::Iana,
        name: "ODX",
        extensions: &[],
        media_types: &["application/ODX"],
        signatures: &[],
        related_formats: &[],
    },
};
