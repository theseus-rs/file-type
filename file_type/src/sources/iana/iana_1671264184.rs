use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1671264184: FileType = FileType {
    file_format: &FileFormat {
        id: 1_671_264_184,
        source_type: SourceType::Iana,
        name: "epp+xml",
        extensions: &[],
        media_types: &["application/epp+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
