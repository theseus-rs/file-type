use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854205: FileFormat = FileFormat {
    id: 105_854_205,
    source_type: SourceType::Wikidata,
    name: "Kindle Topaz eBook",
    extensions: &["azw1", "tpz"],
    media_types: &[],
    signatures: &[
        Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x54, 0x50, 0x5A, 0x30])],
                },
            }],
        },
        Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x54, 0x50, 0x5A, 0x30])],
                },
            }],
        },
    ],
    related_formats: &[],
};
