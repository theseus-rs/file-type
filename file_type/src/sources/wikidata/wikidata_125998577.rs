use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_125998577: FileFormat = FileFormat {
    id: 125_998_577,
    source_type: SourceType::Wikidata,
    name: "Sibelius Score 5",
    extensions: &["sib"],
    media_types: &["application/x-sibelius-score"],
    internal_signatures: &[],
    related_formats: &[],
};
