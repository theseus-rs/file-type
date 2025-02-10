use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3686060264: FileType = FileType {
    file_format: &FileFormat {
        id: 3_686_060_264,
        source_type: SourceType::Iana,
        name: "vnd.ms-wmdrm.lic-chlg-req",
        extensions: &[],
        media_types: &["application/vnd.ms-wmdrm.lic-chlg-req"],
        signatures: &[],
        related_formats: &[],
    },
};
