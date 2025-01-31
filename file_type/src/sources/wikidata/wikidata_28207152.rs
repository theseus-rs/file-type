use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28207152: FileFormat = FileFormat {
    id: 28_207_152,
    puid: "wikidata/28207152",
    name: "PTG",
    extensions: &["ptg"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
