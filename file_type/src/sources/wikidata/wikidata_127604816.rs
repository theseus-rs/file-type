use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_127604816: FileFormat = FileFormat {
    id: 127_604_816,
    puid: "wikidata/127604816",
    name: "AMPL model file",
    extensions: &["mod"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
