use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_1825399547: FileFormat = FileFormat {
    id: 1_825_399_547,
    source_type: SourceType::Httpd,
    name: "mj2",
    extensions: &["mj2", "mjp2"],
    media_types: &["video/mj2"],
    internal_signatures: &[],
    related_formats: &[],
};
