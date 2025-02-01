use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_113438108: FileFormat = FileFormat {
    id: 113_438_108,
    puid: "wikidata/113438108",
    name: "EndNote Library X - X9",
    extensions: &["enl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
