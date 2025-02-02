use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3212196054821300571: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "cdlink",
    extensions: &["vcd"],
    media_types: &["application/x-cdlink"],
    internal_signatures: &[],
    related_formats: &[],
};
