use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3352933827: FileType = FileType {
    file_format: &FileFormat {
        id: 3_352_933_827,
        source_type: SourceType::Iana,
        name: "vnd.llamagraphics.life-balance.desktop",
        extensions: &[],
        media_types: &["application/vnd.llamagraphics.life-balance.desktop"],
        signatures: &[],
        related_formats: &[],
    },
};
