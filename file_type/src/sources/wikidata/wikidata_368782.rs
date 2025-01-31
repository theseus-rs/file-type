use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_368782: FileFormat = FileFormat {
    id: 368_782,
    puid: "wikidata/368782",
    name: "LHA",
    extensions: &["lha", "lzh"],
    media_types: &[
        "application/x-lzh-compressed",
        "application/x-lzh-compressed",
    ],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x2D, 0x6C, 0x68])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x2D, 0x6C, 0x68])],
                },
            }],
        },
    ],
    related_formats: &[],
};
