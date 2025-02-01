use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_29905206: FileFormat = FileFormat {
    id: 29_905_206,
    puid: "wikidata/29905206",
    name: "Self-Dissolving Archive",
    extensions: &["sda"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
