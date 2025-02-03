use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3427035306: FileFormat = FileFormat {
    id: 3_427_035_306,
    source_type: SourceType::Httpd,
    name: "ms wmz",
    extensions: &["wmz"],
    media_types: &["application/x-ms-wmz"],
    internal_signatures: &[],
    related_formats: &[],
};
