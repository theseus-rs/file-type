use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105854712: FileFormat = FileFormat {
    id: 105_854_712,
    puid: "wikidata/105854712",
    name: "OOP compressed archive",
    extensions: &["oop"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xAD, 0x36, 0x22])],
            },
        }],
    }],
    related_formats: &[],
};
