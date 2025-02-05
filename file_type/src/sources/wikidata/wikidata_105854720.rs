use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854720: FileFormat = FileFormat {
    id: 105_854_720,
    source_type: SourceType::Wikidata,
    name: "Liquid Audio",
    extensions: &["lqt"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4C, 0x69, 0x63, 0x6B, 0x00, 0x00, 0x00, 0x02, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
