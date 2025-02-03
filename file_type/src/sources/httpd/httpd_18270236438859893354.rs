use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_18270236438859893354: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "bzip",
    extensions: &["bz"],
    media_types: &["application/x-bzip"],
    internal_signatures: &[],
    related_formats: &[],
};
