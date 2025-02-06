use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3278948029: FileFormat = FileFormat {
    id: 3_278_948_029,
    source_type: SourceType::Iana,
    name: "vnd.fujitsu.oasysprs",
    extensions: &[],
    media_types: &["application/vnd.fujitsu.oasysprs"],
    signatures: &[],
    related_formats: &[],
};
