use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_2626880621: FileType = FileType {
    file_format: &FileFormat {
        id: 2_626_880_621,
        source_type: SourceType::Httpd,
        name: "fluxtime clip",
        extensions: &["ftc"],
        media_types: &["application/vnd.fluxtime.clip"],
        signatures: &[],
        related_formats: &[],
    },
};
