use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_792947184: FileType = FileType {
    file_format: &FileFormat {
        id: 792_947_184,
        source_type: SourceType::Httpd,
        name: "vrml",
        extensions: &["wrl", "vrml"],
        media_types: &["model/vrml"],
        signatures: &[],
        related_formats: &[],
    },
};
