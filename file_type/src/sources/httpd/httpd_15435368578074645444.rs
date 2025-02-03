use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_15435368578074645444: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "marc",
    extensions: &["mrc"],
    media_types: &["application/marc"],
    internal_signatures: &[],
    related_formats: &[],
};
