use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_1593720423: FileType = FileType {
    file_format: &FileFormat {
        id: 1_593_720_423,
        source_type: SourceType::Httpd,
        name: "koan",
        extensions: &["skp", "skd", "skt", "skm"],
        media_types: &["application/vnd.koan"],
        signatures: &[],
        related_formats: &[],
    },
};
