use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_455646521709565751: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "adobe air application installer package zip",
    extensions: &["air"],
    media_types: &["application/vnd.adobe.air-application-installer-package+zip"],
    internal_signatures: &[],
    related_formats: &[],
};
