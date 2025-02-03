use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_7965017775226841149: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "inkml xml",
    extensions: &["ink", "inkml"],
    media_types: &["application/inkml+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
