use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_16688631424076391908: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "syncml xml",
    extensions: &["xsm"],
    media_types: &["application/vnd.syncml+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
