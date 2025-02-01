use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858953: FileFormat = FileFormat {
    id: 105_858_953,
    puid: "wikidata/105858953",
    name: "bigBed Track Format",
    extensions: &["bb", "bigbed"],
    media_types: &["application/octet-stream", "application/octet-stream"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xEB, 0xF2, 0x89, 0x87, 0x04, 0x00])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xEB, 0xF2, 0x89, 0x87, 0x04, 0x00])],
                },
            }],
        },
    ],
    related_formats: &[],
};
