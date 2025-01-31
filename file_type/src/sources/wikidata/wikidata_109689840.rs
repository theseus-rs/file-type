use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_109689840: FileFormat = FileFormat {
    id: 109_689_840,
    puid: "wikidata/109689840",
    name: "MeshLab Project",
    extensions: &["mlb", "mlp"],
    media_types: &[],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x3C, 0x21, 0x44, 0x4F, 0x43, 0x54, 0x59, 0x50, 0x45, 0x20, 0x4D, 0x65,
                        0x73, 0x68, 0x4C, 0x61, 0x62, 0x44, 0x6F, 0x63, 0x75, 0x6D, 0x65, 0x6E,
                        0x74, 0x3E,
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
                        0x3C, 0x21, 0x44, 0x4F, 0x43, 0x54, 0x59, 0x50, 0x45, 0x20, 0x4D, 0x65,
                        0x73, 0x68, 0x4C, 0x61, 0x62, 0x44, 0x6F, 0x63, 0x75, 0x6D, 0x65, 0x6E,
                        0x74, 0x3E,
                    ])],
                },
            }],
        },
    ],
    related_formats: &[],
};
