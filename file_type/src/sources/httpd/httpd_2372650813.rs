use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2372650813: FileFormat = FileFormat {
    id: 2_372_650_813,
    source_type: SourceType::Httpd,
    name: "cdmi capability",
    extensions: &["cdmia"],
    media_types: &["application/cdmi-capability"],
    signatures: &[],
    related_formats: &[],
};
