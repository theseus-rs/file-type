use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111417236: FileFormat = FileFormat {
    id: 111_417_236,
    source_type: SourceType::Wikidata,
    name: "C++ Keyboard Script",
    extensions: &["kb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
