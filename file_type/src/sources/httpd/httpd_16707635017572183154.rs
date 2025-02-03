use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_16707635017572183154: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "epson salt",
    extensions: &["slt"],
    media_types: &["application/vnd.epson.salt"],
    internal_signatures: &[],
    related_formats: &[],
};
