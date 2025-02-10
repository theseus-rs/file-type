use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2626880621: FileType = FileType {
    file_format: &FileFormat {
        id: 2_626_880_621,
        source_type: SourceType::Iana,
        name: "vnd.fluxtime.clip",
        extensions: &[],
        media_types: &["application/vnd.fluxtime.clip"],
        signatures: &[],
        related_formats: &[],
    },
};
