use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856130: FileFormat = FileFormat {
    id: 105_856_130,
    puid: "wikidata/105856130",
    name: "Decision/pro project (v1.00)",
    extensions: &["dp"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x33, 0x30, 0x00, 0x33, 0x30, 0x00, 0x38, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
