use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858585: FileFormat = FileFormat {
    id: 105_858_585,
    source_type: SourceType::Wikidata,
    name: "Pebble application Binary",
    extensions: &["bin"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x50, 0x42, 0x4C, 0x41, 0x50, 0x50, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
