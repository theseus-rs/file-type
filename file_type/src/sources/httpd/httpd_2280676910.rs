use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2280676910: FileFormat = FileFormat {
    id: 2_280_676_910,
    source_type: SourceType::Httpd,
    name: "publishare delta tree",
    extensions: &["qps"],
    media_types: &["application/vnd.publishare-delta-tree"],
    internal_signatures: &[],
    related_formats: &[],
};
