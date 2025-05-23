use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_260369007: FileType = FileType {
    file_format: &FileFormat {
        id: 260_369_007,
        source_type: SourceType::Iana,
        name: "sslkeylogfile",
        extensions: &[],
        media_types: &["application/sslkeylogfile"],
        signatures: &[],
        related_formats: &[],
    },
};
