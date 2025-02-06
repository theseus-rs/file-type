use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856563: FileFormat = FileFormat {
    id: 105_856_563,
    source_type: SourceType::Wikidata,
    name: "Enable document (var.2)",
    extensions: &["wpf"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x81, 0xCD, 0xAB])],
            },
        }],
    }],
    related_formats: &[],
};
