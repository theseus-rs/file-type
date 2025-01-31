use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_34748290: FileFormat = FileFormat {
    id: 34_748_290,
    puid: "wikidata/34748290",
    name: "T64",
    extensions: &["t64"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
