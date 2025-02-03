use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105862932: FileFormat = FileFormat {
    id: 105_862_932,
    source_type: SourceType::Wikidata,
    name: "Hippel ST module",
    extensions: &["sog"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4D, 0x4D, 0x4D, 0x45, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
