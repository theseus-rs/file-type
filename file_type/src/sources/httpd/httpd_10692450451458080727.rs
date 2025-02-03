use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_10692450451458080727: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "basic",
    extensions: &["au", "snd"],
    media_types: &["audio/basic"],
    internal_signatures: &[],
    related_formats: &[],
};
