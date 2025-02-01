use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_125041198: FileFormat = FileFormat {
    id: 125_041_198,
    puid: "wikidata/125041198",
    name: "ZynAddSubFX Instrument File",
    extensions: &["xiz"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
