use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856244: FileFormat = FileFormat {
    id: 105_856_244,
    puid: "wikidata/105856244",
    name: "Doge Serialized Object Notation",
    extensions: &["dson"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x73, 0x75, 0x63, 0x68])],
            },
        }],
    }],
    related_formats: &[],
};
