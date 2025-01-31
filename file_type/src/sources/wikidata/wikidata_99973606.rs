use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_99973606: FileFormat = FileFormat {
    id: 99_973_606,
    puid: "wikidata/99973606",
    name: "XDOMEA 2.0.0",
    extensions: &["xml"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
