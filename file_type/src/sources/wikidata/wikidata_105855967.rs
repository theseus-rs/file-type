use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105855967: FileFormat = FileFormat {
    id: 105_855_967,
    source_type: SourceType::Wikidata,
    name: "Windows Minidump",
    extensions: &["dmp", "mdmp"],
    media_types: &[],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4D, 0x44, 0x4D, 0x50, 0x93, 0xA7])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4D, 0x44, 0x4D, 0x50, 0x93, 0xA7])],
                },
            }],
        },
    ],
    related_formats: &[],
};
