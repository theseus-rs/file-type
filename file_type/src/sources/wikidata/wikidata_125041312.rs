use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_125041312: FileFormat = FileFormat {
    id: 125_041_312,
    puid: "wikidata/125041312",
    name: "ZynAddSubFX Presets File",
    extensions: &["xpz"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
