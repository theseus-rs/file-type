use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3057648254: FileFormat = FileFormat {
    id: 3_057_648_254,
    source_type: SourceType::Httpd,
    name: "contact cmsg",
    extensions: &["cdbcmsg"],
    media_types: &["application/vnd.contact.cmsg"],
    internal_signatures: &[],
    related_formats: &[],
};
