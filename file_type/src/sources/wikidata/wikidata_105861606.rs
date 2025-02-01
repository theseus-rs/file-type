use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105861606: FileFormat = FileFormat {
    id: 105_861_606,
    puid: "wikidata/105861606",
    name: "LEN Exchange Format",
    extensions: &["lef"],
    media_types: &["text/xml"],
    internal_signatures: &[],
    related_formats: &[],
};
