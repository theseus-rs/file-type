use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_658956131: FileFormat = FileFormat {
    id: 658_956_131,
    source_type: SourceType::Httpd,
    name: "sun xml impress template",
    extensions: &["sti"],
    media_types: &["application/vnd.sun.xml.impress.template"],
    internal_signatures: &[],
    related_formats: &[],
};
