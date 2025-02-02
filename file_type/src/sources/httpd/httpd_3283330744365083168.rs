use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3283330744365083168: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "cmx",
    extensions: &["cmx"],
    media_types: &["image/x-cmx"],
    internal_signatures: &[],
    related_formats: &[],
};
