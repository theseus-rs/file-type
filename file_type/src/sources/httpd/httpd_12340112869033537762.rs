use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_12340112869033537762: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "mfer",
    extensions: &["mwf"],
    media_types: &["application/vnd.mfer"],
    internal_signatures: &[],
    related_formats: &[],
};
