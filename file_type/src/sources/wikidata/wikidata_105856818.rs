use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105856818: FileFormat = FileFormat {
    id: 105_856_818,
    source_type: SourceType::Wikidata,
    name: "Game Maker 6 project",
    extensions: &["gm6"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x91, 0xD5, 0x12, 0x00, 0x58, 0x02, 0x00, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
