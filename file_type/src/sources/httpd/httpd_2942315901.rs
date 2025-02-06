use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2942315901: FileFormat = FileFormat {
    id: 2_942_315_901,
    source_type: SourceType::Httpd,
    name: "dpgraph",
    extensions: &["dpg"],
    media_types: &["application/vnd.dpgraph"],
    signatures: &[],
    related_formats: &[],
};
