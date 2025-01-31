use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858258: FileFormat = FileFormat {
    id: 105_858_258,
    puid: "wikidata/105858258",
    name: "World Construction Set Elevation data",
    extensions: &["elev"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x3F, 0x82, 0x8F, 0x5C, 0x00, 0x00, 0x01, 0x2C, 0x00, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
