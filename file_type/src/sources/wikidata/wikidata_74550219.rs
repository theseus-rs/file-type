use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_74550219: FileFormat = FileFormat {
    id: 74_550_219,
    puid: "wikidata/74550219",
    name: "Micrografx clipart index",
    extensions: &["sbj"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
