use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_2570068954: FileType = FileType {
    file_format: &FileFormat {
        id: 2_570_068_954,
        source_type: SourceType::Httpd,
        name: "pg format",
        extensions: &["str"],
        media_types: &["application/vnd.pg.format"],
        signatures: &[],
        related_formats: &[],
    },
};
