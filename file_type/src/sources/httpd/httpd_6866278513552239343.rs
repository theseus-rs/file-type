use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_6866278513552239343: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "cbr",
    extensions: &["cbr", "cba", "cbt", "cbz", "cb7"],
    media_types: &["application/x-cbr"],
    internal_signatures: &[],
    related_formats: &[],
};
