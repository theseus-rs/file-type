use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858101: FileFormat = FileFormat {
    id: 105_858_101,
    source_type: SourceType::Wikidata,
    name: "Twist Import script",
    extensions: &["i"],
    media_types: &["text/plain"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x49, 0x4D, 0x50, 0x4F, 0x52, 0x54, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
