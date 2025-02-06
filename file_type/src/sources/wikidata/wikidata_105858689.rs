use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858689: FileFormat = FileFormat {
    id: 105_858_689,
    source_type: SourceType::Wikidata,
    name: "Crack Art bitmap (med-res)",
    extensions: &["ca2"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x43, 0x41, 0x01, 0x01])],
            },
        }],
    }],
    related_formats: &[],
};
