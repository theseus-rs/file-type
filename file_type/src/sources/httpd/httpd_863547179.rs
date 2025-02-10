use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_863547179: FileType = FileType {
    file_format: &FileFormat {
        id: 863_547_179,
        source_type: SourceType::Httpd,
        name: "xml dtd",
        extensions: &["dtd"],
        media_types: &["application/xml-dtd"],
        signatures: &[],
        related_formats: &[],
    },
};
