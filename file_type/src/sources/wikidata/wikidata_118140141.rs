use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_118140141: FileFormat = FileFormat {
    id: 118_140_141,
    puid: "wikidata/118140141",
    name: "Serenade Schematic File",
    extensions: &["sch"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
