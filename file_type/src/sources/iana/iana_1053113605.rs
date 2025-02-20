use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1053113605: FileType = FileType {
    file_format: &FileFormat {
        id: 1_053_113_605,
        source_type: SourceType::Iana,
        name: "vnd.afpc.foca-charset",
        extensions: &[],
        media_types: &["application/vnd.afpc.foca-charset"],
        signatures: &[],
        related_formats: &[],
    },
};
