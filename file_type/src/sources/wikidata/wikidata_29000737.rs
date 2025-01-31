use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_29000737: FileFormat = FileFormat {
    id: 29_000_737,
    puid: "wikidata/29000737",
    name: "Volume data format",
    extensions: &["vol"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
