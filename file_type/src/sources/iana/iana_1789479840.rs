use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1789479840: FileFormat = FileFormat {
    id: 1_789_479_840,
    source_type: SourceType::Iana,
    name: "vnd.vividence.scriptfile",
    extensions: &[],
    media_types: &["application/vnd.vividence.scriptfile"],
    signatures: &[],
    related_formats: &[],
};
