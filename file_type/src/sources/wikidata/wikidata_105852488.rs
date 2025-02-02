use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105852488: FileFormat = FileFormat {
    id: 105_852_488,
    source_type: SourceType::Wikidata,
    name: "Shadowgrounds 3D model",
    extensions: &["s3d"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x53, 0x33, 0x44, 0x30])],
            },
        }],
    }],
    related_formats: &[],
};
