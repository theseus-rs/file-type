use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1650765870: FileFormat = FileFormat {
    id: 1_650_765_870,
    source_type: SourceType::Iana,
    name: "vnd.mfmp",
    extensions: &[],
    media_types: &["application/vnd.mfmp"],
    internal_signatures: &[],
    related_formats: &[],
};
