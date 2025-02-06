use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111417212: FileFormat = FileFormat {
    id: 111_417_212,
    source_type: SourceType::Wikidata,
    name: "Keyboard file",
    extensions: &["kb"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
