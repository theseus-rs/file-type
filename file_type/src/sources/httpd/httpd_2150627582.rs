use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2150627582: FileFormat = FileFormat {
    id: 2_150_627_582,
    source_type: SourceType::Httpd,
    name: "resource lists xml",
    extensions: &["rl"],
    media_types: &["application/resource-lists+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
