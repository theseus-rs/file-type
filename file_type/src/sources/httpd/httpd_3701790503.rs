use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3701790503: FileType = FileType {
    file_format: &FileFormat {
        id: 3_701_790_503,
        source_type: SourceType::Httpd,
        name: "dynageo",
        extensions: &["geo"],
        media_types: &["application/vnd.dynageo"],
        signatures: &[],
        related_formats: &[],
    },
};
