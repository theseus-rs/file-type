use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_1238229: FileFormat = FileFormat {
    id: 1_238_229,
    source_type: SourceType::Wikidata,
    name: "STereoLithography",
    extensions: &["stl"],
    media_types: &["model/stl"],
    signatures: &[
        Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x73, 0x6F, 0x6C, 0x69, 0x64])],
                },
            }],
        },
        Signature {
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
