use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_2500122023: FileType = FileType {
    file_format: &FileFormat {
        id: 2_500_122_023,
        source_type: SourceType::Httpd,
        name: "otf",
        extensions: &["otf"],
        media_types: &["font/otf"],
        signatures: &[],
        related_formats: &[],
    },
};
