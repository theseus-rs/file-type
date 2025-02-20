use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3678127052: FileType = FileType {
    file_format: &FileFormat {
        id: 3_678_127_052,
        source_type: SourceType::Iana,
        name: "vnd.patientecommsdoc",
        extensions: &[],
        media_types: &["application/vnd.patientecommsdoc"],
        signatures: &[],
        related_formats: &[],
    },
};
