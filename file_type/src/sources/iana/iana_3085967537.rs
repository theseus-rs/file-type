use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3085967537: FileType = FileType {
    file_format: &FileFormat {
        id: 3_085_967_537,
        source_type: SourceType::Iana,
        name: "andrew-inset",
        extensions: &[],
        media_types: &["application/andrew-inset"],
        signatures: &[],
        related_formats: &[],
    },
};
