use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_482286266: FileFormat = FileFormat {
    id: 482_286_266,
    source_type: SourceType::Httpd,
    name: "marcxml xml",
    extensions: &["mrcx"],
    media_types: &["application/marcxml+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
