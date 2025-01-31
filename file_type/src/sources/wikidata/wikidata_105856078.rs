use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856078: FileFormat = FileFormat {
    id: 105_856_078,
    puid: "wikidata/105856078",
    name: "DataBase Diagram",
    extensions: &["dbd"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x3C, 0x58, 0x4D, 0x4C, 0x44, 0x69, 0x61, 0x67, 0x72, 0x61, 0x6D, 0x3E,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
