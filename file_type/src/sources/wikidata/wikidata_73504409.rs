use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_73504409: FileFormat = FileFormat {
    id: 73_504_409,
    puid: "wikidata/73504409",
    name: "ABC FlowCharter file format",
    extensions: &["af2", "af3"],
    media_types: &["application/octet-stream", "application/octet-stream"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x00, 0x18, 0x00, 0x4A, 0x46, 0x4F, 0x00])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x00, 0x18, 0x00, 0x4A, 0x46, 0x4F, 0x00])],
                },
            }],
        },
    ],
    related_formats: &[],
};
