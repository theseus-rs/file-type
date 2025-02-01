use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856563: FileFormat = FileFormat {
    id: 105_856_563,
    puid: "wikidata/105856563",
    name: "Enable document (var.2)",
    extensions: &["wpf"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x81, 0xCD, 0xAB])],
            },
        }],
    }],
    related_formats: &[],
};
