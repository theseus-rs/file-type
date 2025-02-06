use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111417236: FileFormat = FileFormat {
    id: 111_417_236,
    source_type: SourceType::Wikidata,
    name: "C++ Keyboard Script",
    extensions: &["kb"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
