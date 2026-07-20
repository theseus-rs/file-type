use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2779306871: FileType = FileType {
    file_format: &FileFormat {
        id: 2_779_306_871,
        source_type: SourceType::Iana,
        name: "vnd.fafa+yaml",
        extensions: &[],
        media_types: &["application/vnd.fafa+yaml"],
        signatures: &[],
        related_formats: &[],
    },
};
