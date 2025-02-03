use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_929475271: FileFormat = FileFormat {
    id: 929_475_271,
    source_type: SourceType::Httpd,
    name: "sun xml impress",
    extensions: &["sxi"],
    media_types: &["application/vnd.sun.xml.impress"],
    internal_signatures: &[],
    related_formats: &[],
};
