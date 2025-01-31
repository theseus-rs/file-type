use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_61774422: FileFormat = FileFormat {
    id: 61_774_422,
    puid: "wikidata/61774422",
    name: "WavPack Correction File, version 5",
    extensions: &["wvc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
