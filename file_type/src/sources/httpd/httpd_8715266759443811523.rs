use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_8715266759443811523: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "noblenet directory",
    extensions: &["nnd"],
    media_types: &["application/vnd.noblenet-directory"],
    internal_signatures: &[],
    related_formats: &[],
};
