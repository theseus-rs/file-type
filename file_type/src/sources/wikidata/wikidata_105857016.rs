use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857016: FileFormat = FileFormat {
    id: 105_857_016,
    puid: "wikidata/105857016",
    name: "Game Doctor SF 7 saved game",
    extensions: &["078"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x47, 0x41, 0x4D, 0x45, 0x20, 0x44, 0x4F, 0x43, 0x54, 0x4F, 0x52, 0x20, 0x53,
                    0x46, 0x20, 0x37,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
