use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105866383: FileFormat = FileFormat {
    id: 105_866_383,
    puid: "wikidata/105866383",
    name: "Pocket Word document (v1)",
    extensions: &["psw", "pwd"],
    media_types: &["application/x-pocket-word", "application/x-pocket-word"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x7B, 0x5C, 0x70, 0x77, 0x64, 0x31])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x7B, 0x5C, 0x70, 0x77, 0x64, 0x31])],
                },
            }],
        },
    ],
    related_formats: &[],
};
