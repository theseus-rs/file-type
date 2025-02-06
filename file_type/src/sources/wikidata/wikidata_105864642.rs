use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105864642: FileFormat = FileFormat {
    id: 105_864_642,
    source_type: SourceType::Wikidata,
    name: "Prophet V preset",
    extensions: &["provbank"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x50, 0x52, 0x4F, 0x50, 0x00, 0x04, 0x30, 0x30, 0x30, 0x30, 0x42, 0x41, 0x4E,
                    0x4B,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
