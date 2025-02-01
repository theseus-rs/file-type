use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105862169: FileFormat = FileFormat {
    id: 105_862_169,
    puid: "wikidata/105862169",
    name: "OctaMED MMD2 module",
    extensions: &["med", "mmd2"],
    media_types: &["audio/x-mod", "audio/x-mod"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4D, 0x4D, 0x44, 0x32])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4D, 0x4D, 0x44, 0x32])],
                },
            }],
        },
    ],
    related_formats: &[],
};
