use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27967389: FileFormat = FileFormat {
    id: 27_967_389,
    puid: "wikidata/27967389",
    name: "Adlib Tracker module",
    extensions: &["sng"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
