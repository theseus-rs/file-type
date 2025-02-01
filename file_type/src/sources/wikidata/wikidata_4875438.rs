use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_4875438: FileFormat = FileFormat {
    id: 4_875_438,
    puid: "wikidata/4875438",
    name: "Be-Music Source",
    extensions: &["bms"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
