use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857234: FileFormat = FileFormat {
    id: 105_857_234,
    source_type: SourceType::Wikidata,
    name: "Home Accounts account",
    extensions: &["ha"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x08, 0x12, 0x55, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x0B,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
