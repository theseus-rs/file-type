use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105865664: FileFormat = FileFormat {
    id: 105_865_664,
    puid: "wikidata/105865664",
    name: "PPMD compressed data",
    extensions: &["pmd"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x8F, 0xAF, 0xAC, 0x84, 0x20, 0x00, 0x00, 0x00, 0x93, 0x80,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
