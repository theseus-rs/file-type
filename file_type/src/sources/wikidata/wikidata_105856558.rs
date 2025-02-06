use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856558: FileFormat = FileFormat {
    id: 105_856_558,
    source_type: SourceType::Wikidata,
    name: "Sharp Wizard data (generic)",
    extensions: &["wzd"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x3C, 0x53, 0x48, 0x41, 0x52, 0x50, 0x20, 0x57, 0x5A, 0x44, 0x20, 0x44, 0x41,
                    0x54, 0x41, 0x3E,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
