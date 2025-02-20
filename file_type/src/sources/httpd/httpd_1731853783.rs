use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_1731853783: FileType = FileType {
    file_format: &FileFormat {
        id: 1_731_853_783,
        source_type: SourceType::Httpd,
        name: "setext",
        extensions: &["etx"],
        media_types: &["text/x-setext"],
        signatures: &[],
        related_formats: &[],
    },
};
