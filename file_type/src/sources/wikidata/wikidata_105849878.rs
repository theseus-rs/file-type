use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105849878: FileFormat = FileFormat {
    id: 105_849_878,
    puid: "wikidata/105849878",
    name: "16bit DOS COM aPACK compressed (v0.96b)",
    extensions: &["com"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0xBE, 0x0D, 0x01, 0xBF, 0x00, 0x70, 0x8B, 0xCF, 0xFC, 0x57, 0xF3, 0xA4, 0xC3,
                    0xBF, 0x00, 0x01, 0x57,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
