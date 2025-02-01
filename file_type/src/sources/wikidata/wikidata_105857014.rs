use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857014: FileFormat = FileFormat {
    id: 105_857_014,
    puid: "wikidata/105857014",
    name: "Gambas form (v3)",
    extensions: &["form"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x23, 0x20, 0x47, 0x61, 0x6D, 0x62, 0x61, 0x73, 0x20, 0x46, 0x6F, 0x72, 0x6D,
                    0x20, 0x46, 0x69, 0x6C, 0x65, 0x20, 0x33, 0x2E, 0x30, 0x0A, 0x0A, 0x7B, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
