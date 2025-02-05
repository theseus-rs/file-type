use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851457: FileFormat = FileFormat {
    id: 105_851_457,
    source_type: SourceType::Wikidata,
    name: "Text - UTF-7 encoded",
    extensions: &["txt"],
    media_types: &["text/plain"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x2B, 0x2F, 0x76])],
            },
        }],
    }],
    related_formats: &[],
};
