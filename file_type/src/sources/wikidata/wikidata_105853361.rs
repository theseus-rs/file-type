use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105853361: FileFormat = FileFormat {
    id: 105_853_361,
    puid: "wikidata/105853361",
    name: "SquashSF image file (big endian)",
    extensions: &["sfs", "squashfs"],
    media_types: &[],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x73, 0x71, 0x73, 0x68])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x73, 0x71, 0x73, 0x68])],
                },
            }],
        },
    ],
    related_formats: &[],
};
