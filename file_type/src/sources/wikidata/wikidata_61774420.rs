use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_61774420: FileFormat = FileFormat {
    id: 61_774_420,
    puid: "wikidata/61774420",
    name: "WavPack Correction File, version 4",
    extensions: &["wvc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
