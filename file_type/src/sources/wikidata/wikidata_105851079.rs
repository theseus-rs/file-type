use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105851079: FileFormat = FileFormat {
    id: 105_851_079,
    puid: "wikidata/105851079",
    name: "Easy Working: The Planner spreadsheet (v2.10)",
    extensions: &["tpl"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x01, 0x00, 0x01, 0x00, 0x37, 0x00, 0x42, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFA, 0x00, 0x0C, 0xFA, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
