use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105861595: FileFormat = FileFormat {
    id: 105_861_595,
    source_type: SourceType::Wikidata,
    name: "Lionheart module",
    extensions: &["lion"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x00, 0x00, 0x00, 0x28, 0x00, 0x00, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
