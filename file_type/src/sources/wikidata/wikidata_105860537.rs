use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860537: FileFormat = FileFormat {
    id: 105_860_537,
    source_type: SourceType::Wikidata,
    name: "RPG Maker VX encrypted Archive (v3)",
    extensions: &["rgss3a"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x52, 0x47, 0x53, 0x53, 0x41, 0x44, 0x00, 0x03,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
