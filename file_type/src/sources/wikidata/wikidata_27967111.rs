use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27967111: FileFormat = FileFormat {
    id: 27_967_111,
    puid: "wikidata/27967111",
    name: "Aley's Module",
    extensions: &["alm", "alm"],
    media_types: &[],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x41, 0x6C, 0x65, 0x79, 0x4D, 0x6F, 0x64])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x41, 0x6C, 0x65, 0x79, 0x20, 0x4D, 0x6F, 0x64,
                    ])],
                },
            }],
        },
    ],
    related_formats: &[],
};
