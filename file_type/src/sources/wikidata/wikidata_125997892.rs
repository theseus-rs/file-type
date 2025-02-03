use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_125997892: FileFormat = FileFormat {
    id: 125_997_892,
    source_type: SourceType::Wikidata,
    name: "Sibelius Score 3",
    extensions: &["sib"],
    media_types: &["application/x-sibelius-score"],
    internal_signatures: &[],
    related_formats: &[],
};
