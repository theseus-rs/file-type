use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2976010174: FileFormat = FileFormat {
    id: 2_976_010_174,
    source_type: SourceType::Httpd,
    name: "mswrite",
    extensions: &["wri"],
    media_types: &["application/x-mswrite"],
    internal_signatures: &[],
    related_formats: &[],
};
