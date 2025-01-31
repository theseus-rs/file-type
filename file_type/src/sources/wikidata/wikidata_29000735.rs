use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_29000735: FileFormat = FileFormat {
    id: 29_000_735,
    puid: "wikidata/29000735",
    name: "VOL",
    extensions: &["vol"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
