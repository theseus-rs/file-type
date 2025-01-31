use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105849617: FileFormat = FileFormat {
    id: 105_849_617,
    puid: "wikidata/105849617",
    name: "WhatsApp encrypted database",
    extensions: &["crypt7"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x00, 0x01, 0x01])],
            },
        }],
    }],
    related_formats: &[],
};
