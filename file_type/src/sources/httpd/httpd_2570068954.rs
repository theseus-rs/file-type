use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2570068954: FileFormat = FileFormat {
    id: 2_570_068_954,
    source_type: SourceType::Httpd,
    name: "pg format",
    extensions: &["str"],
    media_types: &["application/vnd.pg.format"],
    signatures: &[],
    related_formats: &[],
};
