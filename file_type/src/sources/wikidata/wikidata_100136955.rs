use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_100136955: FileFormat = FileFormat {
    id: 100_136_955,
    puid: "wikidata/100136955",
    name: "XDOMEA 2.3.0",
    extensions: &["xml"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
