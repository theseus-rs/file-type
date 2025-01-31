use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855080: FileFormat = FileFormat {
    id: 105_855_080,
    puid: "wikidata/105855080",
    name: "GNAT Ada Library Information",
    extensions: &["ali"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x56, 0x20, 0x22, 0x47, 0x4E, 0x41, 0x54, 0x20, 0x4C, 0x69, 0x62, 0x20, 0x76,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
