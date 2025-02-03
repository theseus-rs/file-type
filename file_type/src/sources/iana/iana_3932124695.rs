use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3932124695: FileFormat = FileFormat {
    id: 3_932_124_695,
    source_type: SourceType::Iana,
    name: "vnd.dzr",
    extensions: &[],
    media_types: &["application/vnd.dzr"],
    internal_signatures: &[],
    related_formats: &[],
};
