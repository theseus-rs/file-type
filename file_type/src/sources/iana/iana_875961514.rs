use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_875961514: FileType = FileType {
    file_format: &FileFormat {
        id: 875_961_514,
        source_type: SourceType::Iana,
        name: "application/jwk-set+jwt",
        extensions: &[],
        media_types: &["application/jwk-set+jwt"],
        signatures: &[],
        related_formats: &[],
    },
};
