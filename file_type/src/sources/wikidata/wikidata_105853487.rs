use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105853487: FileFormat = FileFormat {
    id: 105_853_487,
    source_type: SourceType::Wikidata,
    name: "Inno Setup data",
    extensions: &["bin", "dat"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x7A, 0x6C, 0x62, 0x1A])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x7A, 0x6C, 0x62, 0x1A])],
                },
            }],
        },
    ],
    related_formats: &[],
};
