use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105862610: FileFormat = FileFormat {
    id: 105_862_610,
    puid: "wikidata/105862610",
    name: "Gateway Music",
    extensions: &["mus"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xA1, 0x05, 0x00, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
