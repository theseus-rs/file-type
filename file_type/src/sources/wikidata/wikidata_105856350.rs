use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856350: FileFormat = FileFormat {
    id: 105_856_350,
    source_type: SourceType::Wikidata,
    name: "Draft Choice for Windows drawing",
    extensions: &["dcw"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x0E, 0x44, 0x43, 0x57, 0x49, 0x4E, 0x20, 0x44, 0x72, 0x61, 0x77, 0x69, 0x6E,
                    0x67, 0x1A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
