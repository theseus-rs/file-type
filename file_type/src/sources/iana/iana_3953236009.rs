use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3953236009: FileType = FileType {
    file_format: &FileFormat {
        id: 3_953_236_009,
        source_type: SourceType::Iana,
        name: "vnd.sri",
        extensions: &[],
        media_types: &["application/vnd.sri"],
        signatures: &[],
        related_formats: &[],
    },
};
