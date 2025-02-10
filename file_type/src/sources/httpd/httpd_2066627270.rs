use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_2066627270: FileType = FileType {
    file_format: &FileFormat {
        id: 2_066_627_270,
        source_type: SourceType::Httpd,
        name: "mif",
        extensions: &["mif"],
        media_types: &["application/vnd.mif"],
        signatures: &[],
        related_formats: &[],
    },
};
