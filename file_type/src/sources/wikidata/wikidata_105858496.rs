use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858496: FileFormat = FileFormat {
    id: 105_858_496,
    source_type: SourceType::Wikidata,
    name: "AmigaBASIC source",
    extensions: &["bas"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xF5, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
