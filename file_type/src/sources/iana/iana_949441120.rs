use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_949441120: FileType = FileType {
    file_format: &FileFormat {
        id: 949_441_120,
        source_type: SourceType::Iana,
        name: "vnd.openxmlformats-officedocument.customXmlProperties+xml",
        extensions: &[],
        media_types: &["application/vnd.openxmlformats-officedocument.customXmlProperties+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
