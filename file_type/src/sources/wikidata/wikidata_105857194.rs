use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105857194: FileFormat = FileFormat {
    id: 105_857_194,
    source_type: SourceType::Wikidata,
    name: "PlayStation Hierarchical 3D Model Data",
    extensions: &["hmd"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x00, 0x00, 0x00, 0x50])],
            },
        }],
    }],
    related_formats: &[],
};
