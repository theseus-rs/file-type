use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_113644918: FileFormat = FileFormat {
    id: 113_644_918,
    puid: "wikidata/113644918",
    name: "Intel SatisFAXtion",
    extensions: &["dcx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
