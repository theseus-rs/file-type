use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_123378444: FileFormat = FileFormat {
    id: 123_378_444,
    puid: "wikidata/123378444",
    name: "Caligari Amiga file",
    extensions: &["sob"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
