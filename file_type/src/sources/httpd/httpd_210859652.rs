use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_210859652: FileFormat = FileFormat {
    id: 210_859_652,
    source_type: SourceType::Httpd,
    name: "fuzzysheet",
    extensions: &["fzs"],
    media_types: &["application/vnd.fuzzysheet"],
    internal_signatures: &[],
    related_formats: &[],
};
