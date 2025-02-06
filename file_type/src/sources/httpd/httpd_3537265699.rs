use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3537265699: FileFormat = FileFormat {
    id: 3_537_265_699,
    source_type: SourceType::Httpd,
    name: "lotus freelance",
    extensions: &["pre"],
    media_types: &["application/vnd.lotus-freelance"],
    signatures: &[],
    related_formats: &[],
};
