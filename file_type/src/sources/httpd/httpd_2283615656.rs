use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_2283615656: FileType = FileType {
    file_format: &FileFormat {
        id: 2_283_615_656,
        source_type: SourceType::Httpd,
        name: "nuera ecelp4800",
        extensions: &["ecelp4800"],
        media_types: &["audio/vnd.nuera.ecelp4800"],
        signatures: &[],
        related_formats: &[],
    },
};
