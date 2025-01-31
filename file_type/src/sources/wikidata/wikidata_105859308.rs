use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105859308: FileFormat = FileFormat {
    id: 105_859_308,
    puid: "wikidata/105859308",
    name: "Koala Micro Illustrator bitmap",
    extensions: &["pic"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xFF, 0x80, 0xC9, 0xC7, 0x1A])],
            },
        }],
    }],
    related_formats: &[],
};
