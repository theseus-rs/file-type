use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105860213: FileFormat = FileFormat {
    id: 105_860_213,
    source_type: SourceType::Wikidata,
    name: "RandyTab guitar tablature",
    extensions: &["rtab"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x05, 0x32, 0x52, 0x54, 0x46, 0x46])],
            },
        }],
    }],
    related_formats: &[],
};
