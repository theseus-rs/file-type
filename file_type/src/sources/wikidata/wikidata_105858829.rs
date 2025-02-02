use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105858829: FileFormat = FileFormat {
    id: 105_858_829,
    source_type: SourceType::Wikidata,
    name: "Multipaint image (Plus4 multicolor)",
    extensions: &["bin"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x06, 0x00, 0x3D, 0x13, 0x78, 0x28, 0x00, 0x19, 0x00, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
