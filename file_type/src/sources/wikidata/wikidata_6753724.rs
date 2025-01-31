use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_6753724: FileFormat = FileFormat {
    id: 6_753_724,
    puid: "wikidata/6753724",
    name: "MapInfo TAB format",
    extensions: &["tab"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
