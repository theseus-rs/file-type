use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_14360260915108678098: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "intergeo",
    extensions: &["i2g"],
    media_types: &["application/vnd.intergeo"],
    internal_signatures: &[],
    related_formats: &[],
};
