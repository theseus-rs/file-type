use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854319: FileFormat = FileFormat {
    id: 105_854_319,
    source_type: SourceType::Wikidata,
    name: "Top 4 compressed data",
    extensions: &["t4", "to4"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x54, 0x34, 0x1A, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
