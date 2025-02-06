use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_125041198: FileFormat = FileFormat {
    id: 125_041_198,
    source_type: SourceType::Wikidata,
    name: "ZynAddSubFX Instrument File",
    extensions: &["xiz"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
