use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3603688327: FileFormat = FileFormat {
    id: 3_603_688_327,
    source_type: SourceType::Iana,
    name: "vnd.evolv.ecig.profile",
    extensions: &[],
    media_types: &["application/vnd.evolv.ecig.profile"],
    internal_signatures: &[],
    related_formats: &[],
};
