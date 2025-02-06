use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2735829481: FileFormat = FileFormat {
    id: 2_735_829_481,
    source_type: SourceType::Httpd,
    name: "quicktime",
    extensions: &["qt", "mov"],
    media_types: &["video/quicktime"],
    signatures: &[],
    related_formats: &[],
};
