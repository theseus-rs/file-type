use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105858553: FileFormat = FileFormat {
    id: 105_858_553,
    source_type: SourceType::Wikidata,
    name: "Electone Registrations",
    extensions: &["b00"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xF0, 0x43, 0x70, 0x78, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
