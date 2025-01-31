use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105851143: FileFormat = FileFormat {
    id: 105_851_143,
    puid: "wikidata/105851143",
    name: "Tochal Package",
    extensions: &["tpk"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x54, 0x6F, 0x63, 0x68, 0x61, 0x6C, 0x20, 0x50, 0x61, 0x63, 0x6B, 0x61, 0x67,
                    0x65, 0x20, 0x46, 0x69, 0x6C, 0x65,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
