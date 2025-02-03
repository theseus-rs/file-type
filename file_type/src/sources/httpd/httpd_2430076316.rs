use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2430076316: FileFormat = FileFormat {
    id: 2_430_076_316,
    source_type: SourceType::Httpd,
    name: "fujitsu oasys",
    extensions: &["oas"],
    media_types: &["application/vnd.fujitsu.oasys"],
    internal_signatures: &[],
    related_formats: &[],
};
