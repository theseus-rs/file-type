use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105849280: FileFormat = FileFormat {
    id: 105_849_280,
    puid: "wikidata/105849280",
    name: "YACE64 saved state",
    extensions: &["yace"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x20, 0x20, 0x20, 0x3E, 0x41, 0x70, 0x70, 0x6C, 0x69, 0x63, 0x61, 0x74, 0x69,
                    0x6F, 0x6E,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
