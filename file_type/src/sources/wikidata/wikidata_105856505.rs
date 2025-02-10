use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856505: FileFormat = FileFormat {
    id: 105_856_505,
    source_type: SourceType::Wikidata,
    name: "WordStar 6 document",
    extensions: &["doc", "ws6"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x1D, 0x7D, 0x00, 0x00, 0x60])],
            },
        }],
    }],
    related_formats: &[],
};
