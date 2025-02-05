use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2904866414: FileFormat = FileFormat {
    id: 2_904_866_414,
    source_type: SourceType::Iana,
    name: "vnd.jisp",
    extensions: &[],
    media_types: &["application/vnd.jisp"],
    signatures: &[],
    related_formats: &[],
};
