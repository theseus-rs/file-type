use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_1702047748: FileFormat = FileFormat {
    id: 1_702_047_748,
    source_type: SourceType::Httpd,
    name: "las las xml",
    extensions: &["lasxml"],
    media_types: &["application/vnd.las.las+xml"],
    signatures: &[],
    related_formats: &[],
};
