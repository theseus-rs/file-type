use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853900: FileFormat = FileFormat {
    id: 105_853_900,
    source_type: SourceType::Wikidata,
    name: "ABC FlowCharter Template",
    extensions: &["aft"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x00, 0x18, 0x00, 0x4A, 0x46, 0x4F, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x00, 0x12, 0x03, 0x00, 0x02, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00, 0x01, 0x5A, 0x07, 0x00, 0x00, 0x00, 0x01,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
