use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_125959218: FileFormat = FileFormat {
    id: 125_959_218,
    source_type: SourceType::Wikidata,
    name: "Sibelius Score 2",
    extensions: &["sib"],
    media_types: &["application/x-sibelius-score"],
    internal_signatures: &[],
    related_formats: &[],
};
