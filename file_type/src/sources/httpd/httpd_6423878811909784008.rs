use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_6423878811909784008: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "mods xml",
    extensions: &["mods"],
    media_types: &["application/mods+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
