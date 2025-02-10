use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2091601761: FileType = FileType {
    file_format: &FileFormat {
        id: 2_091_601_761,
        source_type: SourceType::Iana,
        name: "3mf",
        extensions: &[],
        media_types: &["model/3mf"],
        signatures: &[],
        related_formats: &[],
    },
};
