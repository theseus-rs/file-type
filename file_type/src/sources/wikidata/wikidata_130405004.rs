use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_130405004: FileFormat = FileFormat {
    id: 130_405_004,
    puid: "wikidata/130405004",
    name: "Org file",
    extensions: &["org"],
    media_types: &["text/org"],
    internal_signatures: &[],
    related_formats: &[],
};
