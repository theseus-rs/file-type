use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105851389: FileFormat = FileFormat {
    id: 105_851_389,
    source_type: SourceType::Wikidata,
    name: "Truevision3D Model",
    extensions: &["tvm"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x54, 0x56, 0x4D, 0x4F])],
            },
        }],
    }],
    related_formats: &[],
};
