use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_116860218: FileFormat = FileFormat {
    id: 116_860_218,
    puid: "wikidata/116860218",
    name: "Forms Maker & Filler Forms file",
    extensions: &["dtp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
