use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859380: FileFormat = FileFormat {
    id: 105_859_380,
    source_type: SourceType::Wikidata,
    name: "QuickReport Report",
    extensions: &["qrp"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xFF, 0x0A, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
