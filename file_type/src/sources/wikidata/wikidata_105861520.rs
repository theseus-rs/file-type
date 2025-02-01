use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105861520: FileFormat = FileFormat {
    id: 105_861_520,
    puid: "wikidata/105861520",
    name: "Virtual Villagers data",
    extensions: &["ldw"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x6C, 0x64, 0x77, 0x67])],
            },
        }],
    }],
    related_formats: &[],
};
