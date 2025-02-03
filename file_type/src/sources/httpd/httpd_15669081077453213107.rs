use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_15669081077453213107: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "gpx xml",
    extensions: &["gpx"],
    media_types: &["application/gpx+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
