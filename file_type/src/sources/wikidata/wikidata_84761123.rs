use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_84761123: FileFormat = FileFormat {
    id: 84_761_123,
    puid: "wikidata/84761123",
    name: ".gn",
    extensions: &["gn"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
