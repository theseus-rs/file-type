use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_3352933827: FileType = FileType {
    file_format: &FileFormat {
        id: 3_352_933_827,
        source_type: SourceType::Httpd,
        name: "llamagraphics life balance desktop",
        extensions: &["lbd"],
        media_types: &["application/vnd.llamagraphics.life-balance.desktop"],
        signatures: &[],
        related_formats: &[],
    },
};
