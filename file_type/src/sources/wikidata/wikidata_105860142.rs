use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105860142: FileFormat = FileFormat {
    id: 105_860_142,
    source_type: SourceType::Wikidata,
    name: "Rocket eBook (variant)",
    extensions: &["rb"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xB0, 0x0C, 0xF0, 0x0D])],
            },
        }],
    }],
    related_formats: &[],
};
