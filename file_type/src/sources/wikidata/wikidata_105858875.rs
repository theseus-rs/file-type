use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105858875: FileFormat = FileFormat {
    id: 105_858_875,
    source_type: SourceType::Wikidata,
    name: "FaceSaver bitmap",
    extensions: &["fac", "face"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x46, 0x69, 0x72, 0x73, 0x74, 0x4E, 0x61, 0x6D, 0x65, 0x3A,
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
                        0x46, 0x69, 0x72, 0x73, 0x74, 0x4E, 0x61, 0x6D, 0x65, 0x3A,
                    ])],
                },
            }],
        },
    ],
    related_formats: &[],
};
