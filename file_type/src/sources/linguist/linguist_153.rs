use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_153: FileFormat = FileFormat {
    id: 153,
    source_type: SourceType::Linguist,
    name: "Hack",
    extensions: &["hack", "hh", "hhi", "php"],
    media_types: &["application/x-httpd-php"],
    internal_signatures: &[],
    related_formats: &[],
};
