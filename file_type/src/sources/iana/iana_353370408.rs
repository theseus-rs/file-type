use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_353370408: FileFormat = FileFormat {
    id: 353_370_408,
    source_type: SourceType::Iana,
    name: "vnd.nato.openxmlformats-package.iepd+zip",
    extensions: &[],
    media_types: &["application/vnd.nato.openxmlformats-package.iepd+zip"],
    internal_signatures: &[],
    related_formats: &[],
};
