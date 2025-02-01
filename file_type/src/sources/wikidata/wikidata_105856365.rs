use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856365: FileFormat = FileFormat {
    id: 105_856_365,
    puid: "wikidata/105856365",
    name: "LiteDB DataBase",
    extensions: &["db"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x00, 0x00, 0x00, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
