use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856244: FileFormat = FileFormat {
    id: 105_856_244,
    source_type: SourceType::Wikidata,
    name: "Doge Serialized Object Notation",
    extensions: &["dson"],
    media_types: &["text/plain"],
    signatures: &[Signature {
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
