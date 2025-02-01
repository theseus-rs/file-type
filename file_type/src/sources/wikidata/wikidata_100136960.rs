use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_100136960: FileFormat = FileFormat {
    id: 100_136_960,
    puid: "wikidata/100136960",
    name: "XDOMEA 2.4.0",
    extensions: &["xml"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
