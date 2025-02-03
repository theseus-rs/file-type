use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_6100025454960806313: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "dpgraph",
    extensions: &["dpg"],
    media_types: &["application/vnd.dpgraph"],
    internal_signatures: &[],
    related_formats: &[],
};
