use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105850049: FileFormat = FileFormat {
    id: 105_850_049,
    puid: "wikidata/105850049",
    name: "Crystal Alien Map Maker project (JSON)",
    extensions: &["camm"],
    media_types: &["text/json"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0xEF, 0xBB, 0xBF, 0x7B, 0x0A, 0x20, 0x20, 0x22, 0x46, 0x6F, 0x72, 0x6D, 0x61,
                    0x74, 0x22, 0x3A, 0x20, 0x37, 0x2C, 0x0A, 0x20, 0x20, 0x22, 0x54, 0x69, 0x74,
                    0x6C, 0x65, 0x22, 0x3A, 0x20, 0x22,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
