use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856720: FileFormat = FileFormat {
    id: 105_856_720,
    puid: "wikidata/105856720",
    name: "Universal Data Format",
    extensions: &["udf"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x24, 0x44, 0x4F, 0x43])],
            },
        }],
    }],
    related_formats: &[],
};
