use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_958964055: FileType = FileType {
    file_format: &FileFormat {
        id: 958_964_055,
        source_type: SourceType::Httpd,
        name: "bzip",
        extensions: &["bz"],
        media_types: &["application/x-bzip"],
        signatures: &[],
        related_formats: &[],
    },
};
