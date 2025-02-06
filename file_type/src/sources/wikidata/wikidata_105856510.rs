use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856510: FileFormat = FileFormat {
    id: 105_856_510,
    source_type: SourceType::Wikidata,
    name: "Enable document",
    extensions: &["wpf"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x01, 0xCD, 0xAB])],
            },
        }],
    }],
    related_formats: &[],
};
