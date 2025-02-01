use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_1238229: FileFormat = FileFormat {
    id: 1_238_229,
    puid: "wikidata/1238229",
    name: "STereoLithography",
    extensions: &["stl", "stl"],
    media_types: &["model/stl", "model/stl"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x73, 0x6F, 0x6C, 0x69, 0x64])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x73, 0x6F, 0x6C, 0x69, 0x64])],
                },
            }],
        },
    ],
    related_formats: &[],
};
