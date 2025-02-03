use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2472612683: FileFormat = FileFormat {
    id: 2_472_612_683,
    source_type: SourceType::Iana,
    name: "vnd.accpac.simply.aso",
    extensions: &[],
    media_types: &["application/vnd.accpac.simply.aso"],
    internal_signatures: &[],
    related_formats: &[],
};
