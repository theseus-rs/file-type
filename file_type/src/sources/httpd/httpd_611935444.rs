use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_611935444: FileFormat = FileFormat {
    id: 611_935_444,
    source_type: SourceType::Httpd,
    name: "pascal",
    extensions: &["p", "pas"],
    media_types: &["text/x-pascal"],
    signatures: &[],
    related_formats: &[],
};
