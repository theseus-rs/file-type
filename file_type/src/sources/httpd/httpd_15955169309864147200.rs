use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_15955169309864147200: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "graphviz",
    extensions: &["gv"],
    media_types: &["text/vnd.graphviz"],
    internal_signatures: &[],
    related_formats: &[],
};
