use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_269849346801687012: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "wspolicy xml",
    extensions: &["wspolicy"],
    media_types: &["application/wspolicy+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
