use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2099896127: FileType = FileType {
    file_format: &FileFormat {
        id: 2_099_896_127,
        source_type: SourceType::Iana,
        name: "explicit-registration-response+jwt",
        extensions: &[],
        media_types: &["application/explicit-registration-response+jwt"],
        signatures: &[],
        related_formats: &[],
    },
};
