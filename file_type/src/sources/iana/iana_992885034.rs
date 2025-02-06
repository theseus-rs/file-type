use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_992885034: FileFormat = FileFormat {
    id: 992_885_034,
    source_type: SourceType::Iana,
    name: "vnd.gov.sk.e-form+zip",
    extensions: &[],
    media_types: &["application/vnd.gov.sk.e-form+zip"],
    signatures: &[],
    related_formats: &[],
};
