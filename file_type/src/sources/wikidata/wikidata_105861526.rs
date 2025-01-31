use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105861526: FileFormat = FileFormat {
    id: 105_861_526,
    puid: "wikidata/105861526",
    name: "CINEMA 4D Layout",
    extensions: &["l4d"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x36, 0x43, 0x34, 0x44, 0x4C, 0x41, 0x59, 0x36, 0x01,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
