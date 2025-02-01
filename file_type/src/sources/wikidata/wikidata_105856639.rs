use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856639: FileFormat = FileFormat {
    id: 105_856_639,
    puid: "wikidata/105856639",
    name: "L3DT Water Map File",
    extensions: &["wmf"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4C, 0x33, 0x44, 0x54, 0x58, 0x02, 0x57, 0x4D, 0x46, 0x5F, 0x76, 0x31, 0x2E,
                    0x30,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
