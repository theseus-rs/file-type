use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105860253: FileFormat = FileFormat {
    id: 105_860_253,
    source_type: SourceType::Wikidata,
    name: "NASCAR Heat game data archive",
    extensions: &["res", "trk"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x30, 0x54, 0x53, 0x52])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x30, 0x54, 0x53, 0x52])],
                },
            }],
        },
    ],
    related_formats: &[],
};
