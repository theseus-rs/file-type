use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105860554: FileFormat = FileFormat {
    id: 105_860_554,
    source_type: SourceType::Wikidata,
    name: "Universal NES Image Format",
    extensions: &["unf", "unif"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x55, 0x4E, 0x49, 0x46])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x55, 0x4E, 0x49, 0x46])],
                },
            }],
        },
    ],
    related_formats: &[],
};
