use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3387561077: FileFormat = FileFormat {
    id: 3_387_561_077,
    source_type: SourceType::Httpd,
    name: "wqd",
    extensions: &["wqd"],
    media_types: &["application/vnd.wqd"],
    signatures: &[],
    related_formats: &[],
};
