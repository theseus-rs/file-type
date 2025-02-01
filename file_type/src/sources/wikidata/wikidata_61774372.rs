use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_61774372: FileFormat = FileFormat {
    id: 61_774_372,
    puid: "wikidata/61774372",
    name: "WavPack Binary",
    extensions: &["wv"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
