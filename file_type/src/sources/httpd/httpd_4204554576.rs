use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_4204554576: FileFormat = FileFormat {
    id: 4_204_554_576,
    source_type: SourceType::Httpd,
    name: "ms project",
    extensions: &["mpp", "mpt"],
    media_types: &["application/vnd.ms-project"],
    internal_signatures: &[],
    related_formats: &[],
};
