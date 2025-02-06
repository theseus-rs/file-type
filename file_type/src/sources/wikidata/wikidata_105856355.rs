use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856355: FileFormat = FileFormat {
    id: 105_856_355,
    source_type: SourceType::Wikidata,
    name: "Action Replay Saved gamestate",
    extensions: &["duc"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x41, 0x52, 0x44, 0x53])],
            },
        }],
    }],
    related_formats: &[],
};
