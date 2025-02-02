use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105862702: FileFormat = FileFormat {
    id: 105_862_702,
    source_type: SourceType::Wikidata,
    name: "OctaMED MMD3 module",
    extensions: &["med", "mmd3"],
    media_types: &["audio/x-mod"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4D, 0x4D, 0x44, 0x33])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4D, 0x4D, 0x44, 0x33])],
                },
            }],
        },
    ],
    related_formats: &[],
};
