use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105853084: FileFormat = FileFormat {
    id: 105_853_084,
    puid: "wikidata/105853084",
    name: "SquashSF image file (little endian)",
    extensions: &["sfs", "squashfs"],
    media_types: &[],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x68, 0x73, 0x71, 0x73])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x68, 0x73, 0x71, 0x73])],
                },
            }],
        },
    ],
    related_formats: &[],
};
