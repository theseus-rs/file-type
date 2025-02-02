use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_51913877: FileFormat = FileFormat {
    id: 51_913_877,
    source_type: SourceType::Wikidata,
    name: "Ami Pro Document",
    extensions: &["sam"],
    media_types: &["application/octet-stream", "application/vnd.lotus-wordpro"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x5B, 0x76, 0x65, 0x72, 0x5D])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x5B, 0x76, 0x65, 0x72, 0x5D])],
                },
            }],
        },
    ],
    related_formats: &[],
};
