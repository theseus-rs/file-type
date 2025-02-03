use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3675306534347999425: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "mynfc",
    extensions: &["taglet"],
    media_types: &["application/vnd.mynfc"],
    internal_signatures: &[],
    related_formats: &[],
};
