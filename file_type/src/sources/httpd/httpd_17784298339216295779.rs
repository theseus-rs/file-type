use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_17784298339216295779: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "contact cmsg",
    extensions: &["cdbcmsg"],
    media_types: &["application/vnd.contact.cmsg"],
    internal_signatures: &[],
    related_formats: &[],
};
