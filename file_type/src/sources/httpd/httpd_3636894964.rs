use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3636894964: FileFormat = FileFormat {
    id: 3_636_894_964,
    source_type: SourceType::Httpd,
    name: "graphviz",
    extensions: &["gv"],
    media_types: &["text/vnd.graphviz"],
    signatures: &[],
    related_formats: &[],
};
