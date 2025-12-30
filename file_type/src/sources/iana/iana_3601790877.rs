use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3601790877: FileType = FileType {
    file_format: &FileFormat {
        id: 3_601_790_877,
        source_type: SourceType::Iana,
        name: "cmw+json",
        extensions: &[],
        media_types: &["application/cmw+json"],
        signatures: &[],
        related_formats: &[],
    },
};
