use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3278948029: FileFormat = FileFormat {
    id: 3_278_948_029,
    source_type: SourceType::Httpd,
    name: "fujitsu oasysprs",
    extensions: &["bh2"],
    media_types: &["application/vnd.fujitsu.oasysprs"],
    internal_signatures: &[],
    related_formats: &[],
};
