use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_126000091: FileFormat = FileFormat {
    id: 126_000_091,
    source_type: SourceType::Wikidata,
    name: "Sibelius Score 8.1-8.5",
    extensions: &["sib"],
    media_types: &["application/x-sibelius-score"],
    internal_signatures: &[],
    related_formats: &[],
};
