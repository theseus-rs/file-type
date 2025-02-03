use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_14282171209386449834: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "solent sdkm xml",
    extensions: &["sdkm", "sdkd"],
    media_types: &["application/vnd.solent.sdkm+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
