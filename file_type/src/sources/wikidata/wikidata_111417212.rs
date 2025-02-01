use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111417212: FileFormat = FileFormat {
    id: 111_417_212,
    puid: "wikidata/111417212",
    name: "Keyboard file",
    extensions: &["kb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
