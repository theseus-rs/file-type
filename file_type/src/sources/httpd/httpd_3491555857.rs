use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3491555857: FileType = FileType {
    file_format: &FileFormat {
        id: 3_491_555_857,
        source_type: SourceType::Httpd,
        name: "cdlink",
        extensions: &["vcd"],
        media_types: &["application/x-cdlink"],
        signatures: &[],
        related_formats: &[],
    },
};
