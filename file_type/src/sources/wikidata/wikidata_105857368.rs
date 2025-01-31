use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857368: FileFormat = FileFormat {
    id: 105_857_368,
    puid: "wikidata/105857368",
    name: "JCL script (with rem)",
    extensions: &["jcl", "jcl"],
    media_types: &["text/plain", "text/x-jcl"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x2F, 0x2F, 0x2A])],
                },
            }],
        },
        InternalSignature {
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
