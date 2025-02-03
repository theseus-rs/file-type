use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3636894964: FileFormat = FileFormat {
    id: 3_636_894_964,
    source_type: SourceType::Iana,
    name: "vnd.graphviz",
    extensions: &[],
    media_types: &["text/vnd.graphviz"],
    internal_signatures: &[],
    related_formats: &[],
};
