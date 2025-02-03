use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_940430676671419282: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "msmoney",
    extensions: &["mny"],
    media_types: &["application/x-msmoney"],
    internal_signatures: &[],
    related_formats: &[],
};
