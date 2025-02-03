use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105857391: FileFormat = FileFormat {
    id: 105_857_391,
    source_type: SourceType::Wikidata,
    name: "JustWrite document",
    extensions: &["jw", "jwt"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x46, 0x46, 0x46, 0x46, 0x49, 0x49, 0x49, 0x49,
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
                        0x46, 0x46, 0x46, 0x46, 0x49, 0x49, 0x49, 0x49,
                    ])],
                },
            }],
        },
    ],
    related_formats: &[],
};
