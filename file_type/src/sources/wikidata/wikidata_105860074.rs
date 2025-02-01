use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105860074: FileFormat = FileFormat {
    id: 105_860_074,
    puid: "wikidata/105860074",
    name: "Id Software RoQ video",
    extensions: &["roq"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x84, 0x10, 0xFF, 0xFF, 0xFF, 0xFF, 0x1E, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
