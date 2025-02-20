use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_792947184: FileType = FileType {
    file_format: &FileFormat {
        id: 792_947_184,
        source_type: SourceType::Iana,
        name: "vrml",
        extensions: &[],
        media_types: &["model/vrml"],
        signatures: &[],
        related_formats: &[],
    },
};
