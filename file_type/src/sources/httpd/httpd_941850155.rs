use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_941850155: FileType = FileType {
    file_format: &FileFormat {
        id: 941_850_155,
        source_type: SourceType::Httpd,
        name: "eva",
        extensions: &["eva"],
        media_types: &["application/x-eva"],
        signatures: &[],
        related_formats: &[],
    },
};
