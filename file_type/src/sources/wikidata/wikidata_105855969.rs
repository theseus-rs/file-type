use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855969: FileFormat = FileFormat {
    id: 105_855_969,
    source_type: SourceType::Wikidata,
    name: "ACECAD DigiMemo Hand Written memo",
    extensions: &["dhw"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x41, 0x43, 0x45, 0x43, 0x41, 0x44, 0x5F, 0x44, 0x49, 0x47, 0x49, 0x4D, 0x45,
                    0x4D, 0x4F, 0x5F, 0x48, 0x41, 0x4E, 0x44, 0x57, 0x52, 0x49, 0x54, 0x49, 0x4E,
                    0x47, 0x5F,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
