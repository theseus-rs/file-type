use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105862622: FileFormat = FileFormat {
    id: 105_862_622,
    puid: "wikidata/105862622",
    name: "Segment Map",
    extensions: &["mps"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
