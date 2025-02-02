use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105858798: FileFormat = FileFormat {
    id: 105_858_798,
    source_type: SourceType::Wikidata,
    name: "Wii Effects Textures",
    extensions: &["breft"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x52, 0x45, 0x46, 0x54])],
            },
        }],
    }],
    related_formats: &[],
};
