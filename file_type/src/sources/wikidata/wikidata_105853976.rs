use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105853976: FileFormat = FileFormat {
    id: 105_853_976,
    puid: "wikidata/105853976",
    name: "Amiga Money budgets (v1)",
    extensions: &["amm"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x41, 0x4D, 0x4D, 0x31, 0x42, 0x55, 0x44, 0x53,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
