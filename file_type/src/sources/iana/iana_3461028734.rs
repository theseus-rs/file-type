use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3461028734: FileType = FileType {
    file_format: &FileFormat {
        id: 3_461_028_734,
        source_type: SourceType::Iana,
        name: "samlmetadata+xml",
        extensions: &[],
        media_types: &["application/samlmetadata+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
