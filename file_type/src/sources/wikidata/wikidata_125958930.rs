use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_125958930: FileFormat = FileFormat {
    id: 125_958_930,
    source_type: SourceType::Wikidata,
    name: "Sibelius Score 1.2",
    extensions: &["sib"],
    media_types: &["application/x-sibelius-score"],
    internal_signatures: &[],
    related_formats: &[],
};
