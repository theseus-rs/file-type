use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_1213743: FileFormat = FileFormat {
    id: 1_213_743,
    puid: "wikidata/1213743",
    name: "NRG",
    extensions: &["nrg"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
