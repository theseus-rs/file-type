use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_17710107248733465858: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "pn realaudio plugin",
    extensions: &["rmp"],
    media_types: &["audio/x-pn-realaudio-plugin"],
    internal_signatures: &[],
    related_formats: &[],
};
