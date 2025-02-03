use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3729972083: FileFormat = FileFormat {
    id: 3_729_972_083,
    source_type: SourceType::Httpd,
    name: "mrsid image",
    extensions: &["sid"],
    media_types: &["image/x-mrsid-image"],
    internal_signatures: &[],
    related_formats: &[],
};
