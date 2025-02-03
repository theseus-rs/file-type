use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2241674470: FileFormat = FileFormat {
    id: 2_241_674_470,
    source_type: SourceType::Iana,
    name: "vnd.motorola.flexsuite.gotap",
    extensions: &[],
    media_types: &["application/vnd.motorola.flexsuite.gotap"],
    internal_signatures: &[],
    related_formats: &[],
};
