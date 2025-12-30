use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3597902011: FileType = FileType {
    file_format: &FileFormat {
        id: 3_597_902_011,
        source_type: SourceType::Iana,
        name: "vp+sd-jwt",
        extensions: &[],
        media_types: &["application/vp+sd-jwt"],
        signatures: &[],
        related_formats: &[],
    },
};
