use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_285666980: FileType = FileType {
    file_format: &FileFormat {
        id: 285_666_980,
        source_type: SourceType::Httpd,
        name: "glulx",
        extensions: &["ulx"],
        media_types: &["application/x-glulx"],
        signatures: &[],
        related_formats: &[],
    },
};
