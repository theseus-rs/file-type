use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28975672: FileFormat = FileFormat {
    id: 28_975_672,
    source_type: SourceType::Wikidata,
    name: "CINEMA 4D",
    extensions: &["c4d"],
    media_types: &[],
    signatures: &[
        Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x43, 0x34, 0x44, 0x43, 0x34, 0x44, 0x36])],
                },
            }],
        },
        Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x43, 0x34, 0x44, 0x43, 0x34, 0x44, 0x36])],
                },
            }],
        },
    ],
    related_formats: &[],
};
