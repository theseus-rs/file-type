use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3085967537: FileType = FileType {
    file_format: &FileFormat {
        id: 3_085_967_537,
        source_type: SourceType::Httpd,
        name: "andrew inset",
        extensions: &["ez"],
        media_types: &["application/andrew-inset"],
        signatures: &[],
        related_formats: &[],
    },
};
