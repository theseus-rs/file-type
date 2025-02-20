use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3431652971: FileType = FileType {
    file_format: &FileFormat {
        id: 3_431_652_971,
        source_type: SourceType::Httpd,
        name: "nuera ecelp7470",
        extensions: &["ecelp7470"],
        media_types: &["audio/vnd.nuera.ecelp7470"],
        signatures: &[],
        related_formats: &[],
    },
};
