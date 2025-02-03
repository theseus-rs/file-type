use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_571581390: FileFormat = FileFormat {
    id: 571_581_390,
    source_type: SourceType::Httpd,
    name: "accpac simply imp",
    extensions: &["imp"],
    media_types: &["application/vnd.accpac.simply.imp"],
    internal_signatures: &[],
    related_formats: &[],
};
