use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_970323759: FileFormat = FileFormat {
    id: 970_323_759,
    source_type: SourceType::Httpd,
    name: "debian package",
    extensions: &["deb", "udeb"],
    media_types: &["application/x-debian-package"],
    signatures: &[],
    related_formats: &[],
};
