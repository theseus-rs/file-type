use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3160205525: FileType = FileType {
    file_format: &FileFormat {
        id: 3_160_205_525,
        source_type: SourceType::Iana,
        name: "index",
        extensions: &[],
        media_types: &["application/index"],
        signatures: &[],
        related_formats: &[],
    },
};
