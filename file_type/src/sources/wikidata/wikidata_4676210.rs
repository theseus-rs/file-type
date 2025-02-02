use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_4676210: FileFormat = FileFormat {
    id: 4_676_210,
    source_type: SourceType::Wikidata,
    name: "Adaptive Multi-Rate audio codec",
    extensions: &["3ga", "amr"],
    media_types: &["audio/AMR"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x23, 0x21, 0x41, 0x4D, 0x52])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x23, 0x21, 0x41, 0x4D, 0x52])],
                },
            }],
        },
    ],
    related_formats: &[],
};
