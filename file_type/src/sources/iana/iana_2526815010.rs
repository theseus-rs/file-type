use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2526815010: FileFormat = FileFormat {
    id: 2_526_815_010,
    source_type: SourceType::Iana,
    name: "vnd.Mobius.PLC",
    extensions: &[],
    media_types: &["application/vnd.Mobius.PLC"],
    signatures: &[],
    related_formats: &[],
};
