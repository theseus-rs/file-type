use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_5838795347239654995: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "cdmi capability",
    extensions: &["cdmia"],
    media_types: &["application/cdmi-capability"],
    internal_signatures: &[],
    related_formats: &[],
};
