use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_125999747: FileFormat = FileFormat {
    id: 125_999_747,
    source_type: SourceType::Wikidata,
    name: "Sibelius Score 7.5-8.0",
    extensions: &["sib"],
    media_types: &["application/x-sibelius-score"],
    internal_signatures: &[],
    related_formats: &[],
};
