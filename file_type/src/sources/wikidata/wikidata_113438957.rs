use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_113438957: FileFormat = FileFormat {
    id: 113_438_957,
    puid: "wikidata/113438957",
    name: "EndNote Library 20",
    extensions: &["enl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
