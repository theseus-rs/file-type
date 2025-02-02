use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105860602: FileFormat = FileFormat {
    id: 105_860_602,
    source_type: SourceType::Wikidata,
    name: "Revolution MetaCard stack",
    extensions: &["livecode", "rev"],
    media_types: &[],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x52, 0x45, 0x56, 0x4F, 0x32, 0x37, 0x30, 0x30,
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
                        0x52, 0x45, 0x56, 0x4F, 0x32, 0x37, 0x30, 0x30,
                    ])],
                },
            }],
        },
    ],
    related_formats: &[],
};
