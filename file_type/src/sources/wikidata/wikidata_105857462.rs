use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857462: FileFormat = FileFormat {
    id: 105_857_462,
    puid: "wikidata/105857462",
    name: "2IMG Universal Format disk image (Apple II)",
    extensions: &["2img", "2mg"],
    media_types: &[],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x32, 0x49, 0x4D, 0x47])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x32, 0x49, 0x4D, 0x47])],
                },
            }],
        },
    ],
    related_formats: &[],
};
