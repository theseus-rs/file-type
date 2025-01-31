use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_100136218: FileFormat = FileFormat {
    id: 100_136_218,
    puid: "wikidata/100136218",
    name: "XDOMEA 2.2.0",
    extensions: &["xml"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
