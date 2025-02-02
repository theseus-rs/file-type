use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105864599: FileFormat = FileFormat {
    id: 105_864_599,
    source_type: SourceType::Wikidata,
    name: "First Choice database",
    extensions: &["fol", "pfs"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x47, 0x45, 0x52, 0x42, 0x49, 0x4C, 0x44, 0x42,
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
                        0x47, 0x45, 0x52, 0x42, 0x49, 0x4C, 0x44, 0x42,
                    ])],
                },
            }],
        },
    ],
    related_formats: &[],
};
