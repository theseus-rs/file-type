use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_4451768154165309081: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "tgif",
    extensions: &["obj"],
    media_types: &["application/x-tgif"],
    internal_signatures: &[],
    related_formats: &[],
};
