use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2052277755356363743: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "collada xml",
    extensions: &["dae"],
    media_types: &["model/vnd.collada+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
