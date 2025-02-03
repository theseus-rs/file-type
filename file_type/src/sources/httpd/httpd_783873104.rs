use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_783873104: FileFormat = FileFormat {
    id: 783_873_104,
    source_type: SourceType::Httpd,
    name: "opml",
    extensions: &["opml"],
    media_types: &["text/x-opml"],
    internal_signatures: &[],
    related_formats: &[],
};
