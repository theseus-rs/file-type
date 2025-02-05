use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_125999747: FileFormat = FileFormat {
    id: 125_999_747,
    source_type: SourceType::Wikidata,
    name: "Sibelius Score 7.5-8.0",
    extensions: &["sib"],
    media_types: &["application/x-sibelius-score"],
    signatures: &[],
    related_formats: &[],
};
