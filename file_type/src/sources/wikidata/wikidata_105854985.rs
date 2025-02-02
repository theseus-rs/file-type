use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105854985: FileFormat = FileFormat {
    id: 105_854_985,
    source_type: SourceType::Wikidata,
    name: "Compute Shaders Archive",
    extensions: &["csa"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x66, 0x43, 0x53, 0x41])],
            },
        }],
    }],
    related_formats: &[],
};
