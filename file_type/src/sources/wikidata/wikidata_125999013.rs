use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_125999013: FileFormat = FileFormat {
    id: 125_999_013,
    source_type: SourceType::Wikidata,
    name: "Sibelius Score 6",
    extensions: &["sib"],
    media_types: &["application/x-sibelius-score"],
    internal_signatures: &[],
    related_formats: &[],
};
