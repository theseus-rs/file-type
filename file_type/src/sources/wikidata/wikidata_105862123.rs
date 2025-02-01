use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105862123: FileFormat = FileFormat {
    id: 105_862_123,
    puid: "wikidata/105862123",
    name: "Magic Camera Effect",
    extensions: &["mce"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4D, 0x43, 0x45, 0x2D, 0x4D, 0x43])],
            },
        }],
    }],
    related_formats: &[],
};
