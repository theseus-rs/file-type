use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856321: FileFormat = FileFormat {
    id: 105_856_321,
    puid: "wikidata/105856321",
    name: "Microsoft Word for DOS Document",
    extensions: &["dcx", "doc"],
    media_types: &["application/octet-stream", "application/octet-stream"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x31, 0xBE, 0x00, 0x00, 0x00, 0xAB])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x31, 0xBE, 0x00, 0x00, 0x00, 0xAB])],
                },
            }],
        },
    ],
    related_formats: &[],
};
