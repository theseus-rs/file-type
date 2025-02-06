use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3958824751: FileFormat = FileFormat {
    id: 3_958_824_751,
    source_type: SourceType::Httpd,
    name: "adobe air application installer package zip",
    extensions: &["air"],
    media_types: &["application/vnd.adobe.air-application-installer-package+zip"],
    signatures: &[],
    related_formats: &[],
};
