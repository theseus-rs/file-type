use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105859920: FileFormat = FileFormat {
    id: 105_859_920,
    puid: "wikidata/105859920",
    name: "SER format video",
    extensions: &["ser"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4C, 0x55, 0x43, 0x41, 0x4D, 0x2D, 0x52, 0x45, 0x43, 0x4F, 0x52, 0x44, 0x45,
                    0x52,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
