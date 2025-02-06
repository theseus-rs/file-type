use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3437709368: FileFormat = FileFormat {
    id: 3_437_709_368,
    source_type: SourceType::Httpd,
    name: "syncml xml",
    extensions: &["xsm"],
    media_types: &["application/vnd.syncml+xml"],
    signatures: &[],
    related_formats: &[],
};
