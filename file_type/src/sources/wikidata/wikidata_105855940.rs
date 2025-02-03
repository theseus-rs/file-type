use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105855940: FileFormat = FileFormat {
    id: 105_855_940,
    source_type: SourceType::Wikidata,
    name: "DCF images container",
    extensions: &["dcf"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x45, 0x4B, 0x49, 0x46])],
            },
        }],
    }],
    related_formats: &[],
};
