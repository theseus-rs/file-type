use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1407868785: FileFormat = FileFormat {
    id: 1_407_868_785,
    source_type: SourceType::Iana,
    name: "vnd.age",
    extensions: &[],
    media_types: &["application/vnd.age"],
    internal_signatures: &[],
    related_formats: &[],
};
