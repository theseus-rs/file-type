use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28975672: FileFormat = FileFormat {
    id: 28_975_672,
    puid: "wikidata/28975672",
    name: "CINEMA 4D",
    extensions: &["c4d", "c4d"],
    media_types: &[],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x43, 0x34, 0x44, 0x43, 0x34, 0x44, 0x36])],
                },
            }],
        },
        InternalSignature {
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
