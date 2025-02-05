use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856101: FileFormat = FileFormat {
    id: 105_856_101,
    source_type: SourceType::Wikidata,
    name: "Dockerfile",
    extensions: &["dockerfile"],
    media_types: &["text/x-dockerfile", "text/x-dockerfile-config"],
    signatures: &[
        Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x46, 0x52, 0x4F, 0x4D, 0x20])],
                },
            }],
        },
        Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x46, 0x52, 0x4F, 0x4D, 0x20])],
                },
            }],
        },
    ],
    related_formats: &[],
};
