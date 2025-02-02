use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_13662767142429837098: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "isac fcs",
    extensions: &["fcs"],
    media_types: &["application/vnd.isac.fcs"],
    internal_signatures: &[],
    related_formats: &[],
};
