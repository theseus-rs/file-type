use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105849908: FileFormat = FileFormat {
    id: 105_849_908,
    puid: "wikidata/105849908",
    name: "Casio Packed format",
    extensions: &["cpk"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x43, 0x41, 0x53, 0x49, 0x4F, 0x2E, 0x24, 0x26, 0x25, 0x2D, 0x5A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
