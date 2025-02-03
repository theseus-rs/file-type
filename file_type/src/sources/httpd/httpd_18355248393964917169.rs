use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_18355248393964917169: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "gmx",
    extensions: &["gmx"],
    media_types: &["application/vnd.gmx"],
    internal_signatures: &[],
    related_formats: &[],
};
