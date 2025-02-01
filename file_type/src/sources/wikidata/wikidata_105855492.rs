use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855492: FileFormat = FileFormat {
    id: 105_855_492,
    puid: "wikidata/105855492",
    name: "FMOD Designer Project",
    extensions: &["fdp"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x3C, 0x70, 0x72, 0x6F, 0x6A, 0x65, 0x63, 0x74, 0x3E, 0x0D, 0x0A, 0x3C, 0x6E,
                    0x61, 0x6D, 0x65, 0x3E,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
