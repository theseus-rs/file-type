use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_29000684: FileFormat = FileFormat {
    id: 29_000_684,
    source_type: SourceType::Wikidata,
    name: "quick3D scene file",
    extensions: &["q3s"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x71, 0x75, 0x69, 0x63, 0x6B, 0x33, 0x44, 0x73,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
