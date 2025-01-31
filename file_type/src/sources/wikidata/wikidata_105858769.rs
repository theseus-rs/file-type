use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858769: FileFormat = FileFormat {
    id: 105_858_769,
    puid: "wikidata/105858769",
    name: "ADEX bitmap",
    extensions: &["img", "rle"],
    media_types: &[],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x50, 0x49, 0x43, 0x54, 0x00, 0x08])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x50, 0x49, 0x43, 0x54, 0x00, 0x08])],
                },
            }],
        },
    ],
    related_formats: &[],
};
