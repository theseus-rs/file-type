use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_73514063: FileFormat = FileFormat {
    id: 73_514_063,
    puid: "wikidata/73514063",
    name: "PlayStation Archive",
    extensions: &["psarc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
