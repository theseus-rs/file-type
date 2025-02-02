use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_7766891016013321425: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "semd",
    extensions: &["semd"],
    media_types: &["application/vnd.semd"],
    internal_signatures: &[],
    related_formats: &[],
};
