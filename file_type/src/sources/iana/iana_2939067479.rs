use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2939067479: FileType = FileType {
    file_format: &FileFormat {
        id: 2_939_067_479,
        source_type: SourceType::Iana,
        name: "gml+xml",
        extensions: &[],
        media_types: &["application/gml+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
