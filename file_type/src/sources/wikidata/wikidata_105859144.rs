use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105859144: FileFormat = FileFormat {
    id: 105_859_144,
    source_type: SourceType::Wikidata,
    name: "Kt Interchange File Format compressed bitmap",
    extensions: &["kif", "kiff"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4B, 0x49, 0x46, 0x46])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4B, 0x49, 0x46, 0x46])],
                },
            }],
        },
    ],
    related_formats: &[],
};
