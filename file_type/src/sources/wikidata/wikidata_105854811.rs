use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854811: FileFormat = FileFormat {
    id: 105_854_811,
    source_type: SourceType::Wikidata,
    name: "Taijin Media Net karaoke song",
    extensions: &["tjn"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x00, 0x03, 0x54, 0x41, 0x49, 0x4A, 0x49, 0x4E, 0x20, 0x4D, 0x45, 0x44, 0x49,
                    0x41, 0x20, 0x43, 0x4F, 0x2E, 0x2C, 0x4C, 0x54, 0x44,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
