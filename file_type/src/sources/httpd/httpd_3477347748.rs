use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_3477347748: FileType = FileType {
    file_format: &FileFormat {
        id: 3_477_347_748,
        source_type: SourceType::Httpd,
        name: "java source",
        extensions: &["java"],
        media_types: &["text/x-java-source"],
        signatures: &[],
        related_formats: &[],
    },
};
