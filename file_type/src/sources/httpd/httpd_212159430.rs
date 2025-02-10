use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_212159430: FileType = FileType {
    file_format: &FileFormat {
        id: 212_159_430,
        source_type: SourceType::Httpd,
        name: "mac compactpro",
        extensions: &["cpt"],
        media_types: &["application/mac-compactpro"],
        signatures: &[],
        related_formats: &[],
    },
};
