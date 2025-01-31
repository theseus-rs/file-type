use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105853316: FileFormat = FileFormat {
    id: 105_853_316,
    puid: "wikidata/105853316",
    name: "SNNS result",
    extensions: &["res"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x53, 0x4E, 0x4E, 0x53, 0x20, 0x72, 0x65, 0x73, 0x75, 0x6C, 0x74, 0x20, 0x66,
                    0x69, 0x6C, 0x65, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
