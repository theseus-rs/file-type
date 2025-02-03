use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1058998624: FileFormat = FileFormat {
    id: 1_058_998_624,
    source_type: SourceType::Iana,
    name: "vnd.motorola.flexsuite",
    extensions: &[],
    media_types: &["application/vnd.motorola.flexsuite"],
    internal_signatures: &[],
    related_formats: &[],
};
