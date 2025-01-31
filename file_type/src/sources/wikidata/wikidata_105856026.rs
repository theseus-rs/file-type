use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856026: FileFormat = FileFormat {
    id: 105_856_026,
    puid: "wikidata/105856026",
    name: "Dungeon Siege 2 data",
    extensions: &["ds2res"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x44, 0x53, 0x67, 0x32, 0x54, 0x61, 0x6E, 0x6B,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
