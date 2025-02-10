use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1628556655: FileType = FileType {
    file_format: &FileFormat {
        id: 1_628_556_655,
        source_type: SourceType::Iana,
        name: "vnd.hyperdrive+json",
        extensions: &[],
        media_types: &["application/vnd.hyperdrive+json"],
        signatures: &[],
        related_formats: &[],
    },
};
