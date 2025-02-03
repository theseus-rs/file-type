use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105858707: FileFormat = FileFormat {
    id: 105_858_707,
    source_type: SourceType::Wikidata,
    name: "Blu-ray Disc Movie Information",
    extensions: &["bdm", "bdmv"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4D, 0x4F, 0x42, 0x4A, 0x30])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4D, 0x4F, 0x42, 0x4A, 0x30])],
                },
            }],
        },
    ],
    related_formats: &[],
};
