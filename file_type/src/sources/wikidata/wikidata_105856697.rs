use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105856697: FileFormat = FileFormat {
    id: 105_856_697,
    source_type: SourceType::Wikidata,
    name: "Universal Hint System (newer format)",
    extensions: &["uhs"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x55, 0x48, 0x53])],
            },
        }],
    }],
    related_formats: &[],
};
