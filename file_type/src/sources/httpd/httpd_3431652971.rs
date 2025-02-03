use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3431652971: FileFormat = FileFormat {
    id: 3_431_652_971,
    source_type: SourceType::Httpd,
    name: "nuera ecelp7470",
    extensions: &["ecelp7470"],
    media_types: &["audio/vnd.nuera.ecelp7470"],
    internal_signatures: &[],
    related_formats: &[],
};
