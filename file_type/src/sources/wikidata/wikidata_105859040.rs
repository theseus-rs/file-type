use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105859040: FileFormat = FileFormat {
    id: 105_859_040,
    puid: "wikidata/105859040",
    name: "Bryce 6 Scene",
    extensions: &["br6"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x42, 0x72, 0x79, 0x63, 0x65, 0x5F, 0x36, 0x2E, 0x30, 0x5F, 0x46, 0x69, 0x6C,
                    0x65, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
