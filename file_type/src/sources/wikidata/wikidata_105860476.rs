use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105860476: FileFormat = FileFormat {
    id: 105_860_476,
    source_type: SourceType::Wikidata,
    name: "PlayStation RSD 3D model info (gen)",
    extensions: &["rsd"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x40, 0x52, 0x53, 0x44])],
            },
        }],
    }],
    related_formats: &[],
};
