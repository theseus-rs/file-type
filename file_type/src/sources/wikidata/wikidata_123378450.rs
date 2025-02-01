use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_123378450: FileFormat = FileFormat {
    id: 123_378_450,
    puid: "wikidata/123378450",
    name: "TrueSpace Selection file",
    extensions: &["sel"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
