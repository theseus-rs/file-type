use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105856641: FileFormat = FileFormat {
    id: 105_856_641,
    source_type: SourceType::Wikidata,
    name: "Weapon definition script",
    extensions: &["weap"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x77, 0x65, 0x61, 0x70, 0x6F, 0x6E, 0x44, 0x65, 0x66,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
