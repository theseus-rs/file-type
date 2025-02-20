use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_649945256: FileType = FileType {
    file_format: &FileFormat {
        id: 649_945_256,
        source_type: SourceType::Iana,
        name: "vnd.frogans.ltf (OBSOLETE)",
        extensions: &[],
        media_types: &["application/vnd.frogans.ltf"],
        signatures: &[],
        related_formats: &[],
    },
};
