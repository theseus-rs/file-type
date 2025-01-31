use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105849993: FileFormat = FileFormat {
    id: 105_849_993,
    puid: "wikidata/105849993",
    name: "CONTEC Logger Binary data",
    extensions: &["clb"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x43, 0x4F, 0x4E, 0x54, 0x45, 0x43, 0x20, 0x44, 0x41, 0x54, 0x41, 0x20, 0x4C,
                    0x4F, 0x47, 0x47, 0x45, 0x52,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
