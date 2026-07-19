use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3412217871: FileType = FileType {
    file_format: &FileFormat {
        id: 3_412_217_871,
        source_type: SourceType::Iana,
        name: "vnd.lukuid.package+zip",
        extensions: &[],
        media_types: &["application/vnd.lukuid.package+zip"],
        signatures: &[],
        related_formats: &[],
    },
};
