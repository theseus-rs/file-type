use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_1432736077: FileType = FileType {
    file_format: &FileFormat {
        id: 1_432_736_077,
        source_type: SourceType::Httpd,
        name: "matroska",
        extensions: &["mka"],
        media_types: &["audio/x-matroska"],
        signatures: &[],
        related_formats: &[],
    },
};
