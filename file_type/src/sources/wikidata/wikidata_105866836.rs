use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105866836: FileFormat = FileFormat {
    id: 105_866_836,
    source_type: SourceType::Wikidata,
    name: "PerFORM compressed database",
    extensions: &["frf", "frl", "frp"],
    media_types: &[],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x50, 0x65, 0x72, 0x46, 0x4F, 0x52, 0x4D, 0x20, 0x63, 0x6F, 0x6D, 0x70,
                        0x72, 0x65, 0x73, 0x73, 0x65, 0x64, 0x20, 0x64, 0x61, 0x74, 0x61, 0x62,
                        0x61, 0x73, 0x65, 0x20,
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
                        0x50, 0x65, 0x72, 0x46, 0x4F, 0x52, 0x4D, 0x20, 0x63, 0x6F, 0x6D, 0x70,
                        0x72, 0x65, 0x73, 0x73, 0x65, 0x64, 0x20, 0x64, 0x61, 0x74, 0x61, 0x62,
                        0x61, 0x73, 0x65, 0x20,
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
                        0x50, 0x65, 0x72, 0x46, 0x4F, 0x52, 0x4D, 0x20, 0x63, 0x6F, 0x6D, 0x70,
                        0x72, 0x65, 0x73, 0x73, 0x65, 0x64, 0x20, 0x64, 0x61, 0x74, 0x61, 0x62,
                        0x61, 0x73, 0x65, 0x20,
                    ])],
                },
            }],
        },
    ],
    related_formats: &[],
};
