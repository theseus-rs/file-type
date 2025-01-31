use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_130602563: FileFormat = FileFormat {
    id: 130_602_563,
    puid: "wikidata/130602563",
    name: "ReasonML file format",
    extensions: &["re", "rei"],
    media_types: &["text/x-reasonml", "text/x-reasonml"],
    internal_signatures: &[],
    related_formats: &[],
};
