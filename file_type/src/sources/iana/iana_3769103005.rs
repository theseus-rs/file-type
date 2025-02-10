use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3769103005: FileType = FileType {
    file_format: &FileFormat {
        id: 3_769_103_005,
        source_type: SourceType::Iana,
        name: "vnd.ms-wmdrm.meter-chlg-req",
        extensions: &[],
        media_types: &["application/vnd.ms-wmdrm.meter-chlg-req"],
        signatures: &[],
        related_formats: &[],
    },
};
