use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105864320: FileFormat = FileFormat {
    id: 105_864_320,
    source_type: SourceType::Wikidata,
    name: "Psion serie 3 Database",
    extensions: &["dbf", "odb"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4F, 0x50, 0x4C, 0x44, 0x61, 0x74, 0x61, 0x62, 0x61, 0x73, 0x65, 0x46,
                        0x69, 0x6C, 0x65, 0x00,
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
                        0x4F, 0x50, 0x4C, 0x44, 0x61, 0x74, 0x61, 0x62, 0x61, 0x73, 0x65, 0x46,
                        0x69, 0x6C, 0x65, 0x00,
                    ])],
                },
            }],
        },
    ],
    related_formats: &[],
};
