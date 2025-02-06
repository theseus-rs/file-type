use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854712: FileFormat = FileFormat {
    id: 105_854_712,
    source_type: SourceType::Wikidata,
    name: "OOP compressed archive",
    extensions: &["oop"],
    media_types: &[],
    signatures: &[Signature {
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
