use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3081340385: FileType = FileType {
    file_format: &FileFormat {
        id: 3_081_340_385,
        source_type: SourceType::Iana,
        name: "vnd.xacml+json",
        extensions: &[],
        media_types: &["application/vnd.xacml+json"],
        signatures: &[],
        related_formats: &[],
    },
};
