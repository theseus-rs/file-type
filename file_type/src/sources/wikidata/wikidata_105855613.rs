use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855613: FileFormat = FileFormat {
    id: 105_855_613,
    puid: "wikidata/105855613",
    name: "OpenStreetMap O5c data",
    extensions: &["o5c"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0xFF, 0xE0, 0x04, 0x6F, 0x35, 0x63, 0x32, 0xDB, 0x12,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
