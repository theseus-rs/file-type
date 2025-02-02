use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105861616: FileFormat = FileFormat {
    id: 105_861_616,
    source_type: SourceType::Wikidata,
    name: "Take Command: 2nd Manassas game data archive",
    extensions: &["dat", "lsl"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x01, 0x01, 0x01, 0x00, 0x00, 0x00])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x01, 0x01, 0x01, 0x00, 0x00, 0x00])],
                },
            }],
        },
    ],
    related_formats: &[],
};
