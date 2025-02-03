use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105854540: FileFormat = FileFormat {
    id: 105_854_540,
    source_type: SourceType::Wikidata,
    name: "ANSI escape sequence text",
    extensions: &["ans", "asc"],
    media_types: &["text/x-ansi"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x1B, 0x5B])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x1B, 0x5B])],
                },
            }],
        },
    ],
    related_formats: &[],
};
