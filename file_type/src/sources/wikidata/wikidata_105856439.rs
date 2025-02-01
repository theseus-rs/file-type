use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856439: FileFormat = FileFormat {
    id: 105_856_439,
    puid: "wikidata/105856439",
    name: "Windows Script File (UTF-8)",
    extensions: &["wsf"],
    media_types: &["text/xml"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xEF, 0xBB, 0xBF, 0x3C])],
            },
        }],
    }],
    related_formats: &[],
};
