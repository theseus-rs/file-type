use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105852746: FileFormat = FileFormat {
    id: 105_852_746,
    puid: "wikidata/105852746",
    name: "Slide saved game",
    extensions: &["sld"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x0D, 0x0A, 0x53, 0x6C, 0x69, 0x64, 0x65, 0x20, 0x31, 0x2E, 0x30, 0x31, 0x77,
                    0x20, 0x67, 0x61, 0x6D, 0x65, 0x66, 0x69, 0x6C, 0x65, 0x0D, 0x0A, 0x0D, 0x0A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
