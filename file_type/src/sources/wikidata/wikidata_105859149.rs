use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859149: FileFormat = FileFormat {
    id: 105_859_149,
    source_type: SourceType::Wikidata,
    name: "STAD hi-res (h.-packed) bitmap",
    extensions: &["pac"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x70, 0x4D, 0x38, 0x35])],
            },
        }],
    }],
    related_formats: &[],
};
