use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858101: FileFormat = FileFormat {
    id: 105_858_101,
    puid: "wikidata/105858101",
    name: "Twist Import script",
    extensions: &["i"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
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
