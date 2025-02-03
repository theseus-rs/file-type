use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_858406622269959928: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "antix game component",
    extensions: &["atx"],
    media_types: &["application/vnd.antix.game-component"],
    internal_signatures: &[],
    related_formats: &[],
};
