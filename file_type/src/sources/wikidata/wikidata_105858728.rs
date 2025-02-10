use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858728: FileFormat = FileFormat {
    id: 105_858_728,
    source_type: SourceType::Wikidata,
    name: "Graph Saurus bitmap (7/8/S)",
    extensions: &["sr7", "sr8", "srs"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xFE, 0x00, 0x00, 0x00, 0xD4, 0x00, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
