use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_3176050: FileFormat = FileFormat {
    id: 3_176_050,
    puid: "wikidata/3176050",
    name: "SubStation Alpha",
    extensions: &["ass", "ssa"],
    media_types: &[],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0xFF, 0xFE, 0x5B, 0x00, 0x53, 0x00, 0x63, 0x00, 0x72, 0x00, 0x69, 0x00,
                        0x70, 0x00, 0x74, 0x00, 0x20, 0x00, 0x49, 0x00, 0x6E, 0x00, 0x66, 0x00,
                        0x6F, 0x00, 0x5D,
                    ])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0xFF, 0xFE, 0x5B, 0x00, 0x53, 0x00, 0x63, 0x00, 0x72, 0x00, 0x69, 0x00,
                        0x70, 0x00, 0x74, 0x00, 0x20, 0x00, 0x49, 0x00, 0x6E, 0x00, 0x66, 0x00,
                        0x6F, 0x00, 0x5D,
                    ])],
                },
            }],
        },
    ],
    related_formats: &[],
};
