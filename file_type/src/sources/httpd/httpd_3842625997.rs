use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3842625997: FileFormat = FileFormat {
    id: 3_842_625_997,
    source_type: SourceType::Httpd,
    name: "msdownload",
    extensions: &["exe", "dll", "com", "bat", "msi"],
    media_types: &["application/x-msdownload"],
    internal_signatures: &[],
    related_formats: &[],
};
