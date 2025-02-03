use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_272: FileFormat = FileFormat {
    id: 272,
    source_type: SourceType::Linguist,
    name: "PHP",
    extensions: &[
        "aw", "ctp", "fcgi", "inc", "php", "php3", "php4", "php5", "phps", "phpt",
    ],
    media_types: &["application/x-httpd-php"],
    internal_signatures: &[],
    related_formats: &[],
};
