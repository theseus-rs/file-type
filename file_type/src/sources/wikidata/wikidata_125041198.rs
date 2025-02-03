use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_125041198: FileFormat = FileFormat {
    id: 125_041_198,
    source_type: SourceType::Wikidata,
    name: "ZynAddSubFX Instrument File",
    extensions: &["xiz"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
