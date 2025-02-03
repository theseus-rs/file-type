use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2918110073: FileFormat = FileFormat {
    id: 2_918_110_073,
    source_type: SourceType::Iana,
    name: "bufr",
    extensions: &[],
    media_types: &["application/bufr"],
    internal_signatures: &[],
    related_formats: &[],
};
