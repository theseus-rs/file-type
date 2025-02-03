use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_1860797218639255065: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "debian package",
    extensions: &["deb", "udeb"],
    media_types: &["application/x-debian-package"],
    internal_signatures: &[],
    related_formats: &[],
};
