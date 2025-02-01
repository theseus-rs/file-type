use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_125298468: FileFormat = FileFormat {
    id: 125_298_468,
    puid: "wikidata/125298468",
    name: "Scribe",
    extensions: &["scr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
