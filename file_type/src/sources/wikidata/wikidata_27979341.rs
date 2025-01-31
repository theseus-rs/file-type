use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27979341: FileFormat = FileFormat {
    id: 27_979_341,
    puid: "wikidata/27979341",
    name: "Unirast",
    extensions: &["urf", "urf"],
    media_types: &["application/octet-stream", "image/urf"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x55, 0x4E, 0x49, 0x52, 0x41, 0x53, 0x54, 0x00,
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
                        0x55, 0x4E, 0x49, 0x52, 0x41, 0x53, 0x54, 0x00,
                    ])],
                },
            }],
        },
    ],
    related_formats: &[],
};
