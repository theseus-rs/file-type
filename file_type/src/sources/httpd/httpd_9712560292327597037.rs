use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_9712560292327597037: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "msdownload",
    extensions: &["exe", "dll", "com", "bat", "msi"],
    media_types: &["application/x-msdownload"],
    internal_signatures: &[],
    related_formats: &[],
};
