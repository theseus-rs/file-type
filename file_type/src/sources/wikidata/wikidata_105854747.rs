use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105854747: FileFormat = FileFormat {
    id: 105_854_747,
    puid: "wikidata/105854747",
    name: "ABC FlowCharter document",
    extensions: &["abc"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x00, 0x38, 0x00, 0x4A, 0x46, 0x4F])],
            },
        }],
    }],
    related_formats: &[],
};
