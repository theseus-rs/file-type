use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856130: FileFormat = FileFormat {
    id: 105_856_130,
    source_type: SourceType::Wikidata,
    name: "Decision/pro project (v1.00)",
    extensions: &["dp"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
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
