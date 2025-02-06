use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858607: FileFormat = FileFormat {
    id: 105_858_607,
    source_type: SourceType::Wikidata,
    name: "Babylon Glossary",
    extensions: &["bgl"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x12, 0x34, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
