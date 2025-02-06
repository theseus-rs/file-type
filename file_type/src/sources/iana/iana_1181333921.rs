use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1181333921: FileFormat = FileFormat {
    id: 1_181_333_921,
    source_type: SourceType::Iana,
    name: "vnd.ves.encrypted",
    extensions: &[],
    media_types: &["application/vnd.ves.encrypted"],
    signatures: &[],
    related_formats: &[],
};
