use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855603: FileFormat = FileFormat {
    id: 105_855_603,
    source_type: SourceType::Wikidata,
    name: "Office Profile-Settings (v1.1)",
    extensions: &["ops"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x00, 0x20, 0x00, 0x00, 0x00, 0x4F, 0x50, 0x53, 0x20, 0x48, 0x65, 0x61, 0x64,
                    0x65, 0x72, 0x20, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E, 0x20, 0x31, 0x2E,
                    0x31, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
