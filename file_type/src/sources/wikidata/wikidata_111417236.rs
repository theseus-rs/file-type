use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111417236: FileFormat = FileFormat {
    id: 111_417_236,
    puid: "wikidata/111417236",
    name: "C++ Keyboard Script",
    extensions: &["kb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
