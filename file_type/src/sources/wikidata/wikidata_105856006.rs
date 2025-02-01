use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856006: FileFormat = FileFormat {
    id: 105_856_006,
    puid: "wikidata/105856006",
    name: "Obitus game Data",
    extensions: &["dat"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x44, 0x41, 0x54, 0x41, 0xF6, 0xF5, 0xF3])],
            },
        }],
    }],
    related_formats: &[],
};
