use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3003002017: FileFormat = FileFormat {
    id: 3_003_002_017,
    source_type: SourceType::Httpd,
    name: "solent sdkm xml",
    extensions: &["sdkm", "sdkd"],
    media_types: &["application/vnd.solent.sdkm+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
