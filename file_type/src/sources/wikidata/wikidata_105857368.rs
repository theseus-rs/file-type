use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857368: FileFormat = FileFormat {
    id: 105_857_368,
    source_type: SourceType::Wikidata,
    name: "JCL script (with rem)",
    extensions: &["jcl"],
    media_types: &["text/plain", "text/x-jcl"],
    signatures: &[
        Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x2F, 0x2F, 0x2A])],
                },
            }],
        },
        Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x2F, 0x2F, 0x2A])],
                },
            }],
        },
    ],
    related_formats: &[],
};
