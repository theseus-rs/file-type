use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4115926298: FileType = FileType {
    file_format: &FileFormat {
        id: 4_115_926_298,
        source_type: SourceType::Iana,
        name: "vnd.sealed.eml",
        extensions: &[],
        media_types: &["application/vnd.sealed.eml"],
        signatures: &[],
        related_formats: &[],
    },
};
