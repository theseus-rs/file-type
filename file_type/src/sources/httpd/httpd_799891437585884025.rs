use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_799891437585884025: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "ipfix",
    extensions: &["ipfix"],
    media_types: &["application/ipfix"],
    internal_signatures: &[],
    related_formats: &[],
};
