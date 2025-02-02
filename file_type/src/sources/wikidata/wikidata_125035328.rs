use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_125035328: FileFormat = FileFormat {
    id: 125_035_328,
    source_type: SourceType::Wikidata,
    name: "TinkerPlots document",
    extensions: &["tp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
