use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105851879: FileFormat = FileFormat {
    id: 105_851_879,
    puid: "wikidata/105851879",
    name: "AmiDraw Drawing (variant)",
    extensions: &["sdw"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x41, 0x4D, 0x49, 0x5F, 0x4D, 0x45, 0x54, 0x41, 0x46, 0x49, 0x4C, 0x45, 0x5F,
                    0x46, 0x4F, 0x52, 0x4D, 0x41, 0x54, 0x20, 0x56, 0x45, 0x52, 0x53, 0x49, 0x4F,
                    0x4E,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
