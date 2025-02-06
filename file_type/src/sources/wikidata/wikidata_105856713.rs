use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856713: FileFormat = FileFormat {
    id: 105_856_713,
    source_type: SourceType::Wikidata,
    name: "WinCC User Archive export",
    extensions: &["uap"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x55, 0x41, 0x50, 0x30, 0x31, 0x30, 0x30])],
            },
        }],
    }],
    related_formats: &[],
};
