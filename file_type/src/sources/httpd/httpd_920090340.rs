use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_920090340: FileFormat = FileFormat {
    id: 920_090_340,
    source_type: SourceType::Httpd,
    name: "tads",
    extensions: &["gam"],
    media_types: &["application/x-tads"],
    internal_signatures: &[],
    related_formats: &[],
};
