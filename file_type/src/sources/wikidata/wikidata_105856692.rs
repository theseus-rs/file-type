use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856692: FileFormat = FileFormat {
    id: 105_856_692,
    puid: "wikidata/105856692",
    name: "UltraDesign Font",
    extensions: &["ufnt"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x7C, 0x07, 0x00, 0x14, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
