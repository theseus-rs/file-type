use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2403846871: FileType = FileType {
    file_format: &FileFormat {
        id: 2_403_846_871,
        source_type: SourceType::Iana,
        name: "vnd.oma-scws-http-request",
        extensions: &[],
        media_types: &["application/vnd.oma-scws-http-request"],
        signatures: &[],
        related_formats: &[],
    },
};
