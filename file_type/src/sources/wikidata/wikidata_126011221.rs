use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_126011221: FileFormat = FileFormat {
    id: 126_011_221,
    source_type: SourceType::Wikidata,
    name: "Sibelius Score 2022.7-2022.11",
    extensions: &["sib"],
    media_types: &["application/x-sibelius-score"],
    internal_signatures: &[],
    related_formats: &[],
};
