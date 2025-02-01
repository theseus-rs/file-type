use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_61774392: FileFormat = FileFormat {
    id: 61_774_392,
    puid: "wikidata/61774392",
    name: "WavPack Correction File",
    extensions: &["wvc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
