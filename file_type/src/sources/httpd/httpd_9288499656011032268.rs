use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_9288499656011032268: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "osgi dp",
    extensions: &["dp"],
    media_types: &["application/vnd.osgi.dp"],
    internal_signatures: &[],
    related_formats: &[],
};
