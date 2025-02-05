use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_15491224: FileFormat = FileFormat {
    id: 15_491_224,
    source_type: SourceType::Httpd,
    name: "gmx",
    extensions: &["gmx"],
    media_types: &["application/vnd.gmx"],
    signatures: &[],
    related_formats: &[],
};
