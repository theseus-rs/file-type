use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_129002196: FileFormat = FileFormat {
    id: 129_002_196,
    source_type: SourceType::Wikidata,
    name: "EBNF file format",
    extensions: &["ebnf"],
    media_types: &["text/x-ebnf"],
    signatures: &[],
    related_formats: &[],
};
