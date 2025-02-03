use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_1314711206: FileFormat = FileFormat {
    id: 1_314_711_206,
    source_type: SourceType::Httpd,
    name: "pvi ptid1",
    extensions: &["ptid"],
    media_types: &["application/vnd.pvi.ptid1"],
    internal_signatures: &[],
    related_formats: &[],
};
