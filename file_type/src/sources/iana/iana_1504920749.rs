use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1504920749: FileFormat = FileFormat {
    id: 1_504_920_749,
    source_type: SourceType::Iana,
    name: "vnd.google-earth.kml+xml",
    extensions: &[],
    media_types: &["application/vnd.google-earth.kml+xml"],
    signatures: &[],
    related_formats: &[],
};
