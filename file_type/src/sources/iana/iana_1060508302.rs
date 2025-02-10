use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1060508302: FileType = FileType {
    file_format: &FileFormat {
        id: 1_060_508_302,
        source_type: SourceType::Iana,
        name: "vp+jwt",
        extensions: &[],
        media_types: &["application/vp+jwt"],
        signatures: &[],
        related_formats: &[],
    },
};
