use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856124: FileFormat = FileFormat {
    id: 105_856_124,
    source_type: SourceType::Wikidata,
    name: "DFF format (v1.0, LE)",
    extensions: &["dff"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x46, 0x46, 0x44, 0x25])],
            },
        }],
    }],
    related_formats: &[],
};
