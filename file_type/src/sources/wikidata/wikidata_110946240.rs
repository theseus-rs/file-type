use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_110946240: FileFormat = FileFormat {
    id: 110_946_240,
    puid: "wikidata/110946240",
    name: "Drools Rule Language",
    extensions: &["drl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
