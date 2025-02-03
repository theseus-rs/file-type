use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_15873555442907056260: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "svg xml",
    extensions: &["svg", "svgz"],
    media_types: &["image/svg+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
