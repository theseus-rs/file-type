use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_12775287888660207002: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "stepmania stepchart",
    extensions: &["sm"],
    media_types: &["application/vnd.stepmania.stepchart"],
    internal_signatures: &[],
    related_formats: &[],
};
