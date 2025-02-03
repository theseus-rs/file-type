use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105862256: FileFormat = FileFormat {
    id: 105_862_256,
    source_type: SourceType::Wikidata,
    name: "MIME HTML archive format (var 2)",
    extensions: &["mht", "mhtml"],
    media_types: &["application/x-mimearchive"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4D, 0x49, 0x4D, 0x45, 0x2D, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E,
                        0x3A, 0x20, 0x31, 0x2E, 0x30,
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
                        0x4D, 0x49, 0x4D, 0x45, 0x2D, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E,
                        0x3A, 0x20, 0x31, 0x2E, 0x30,
                    ])],
                },
            }],
        },
    ],
    related_formats: &[],
};
