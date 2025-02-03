use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105858897: FileFormat = FileFormat {
    id: 105_858_897,
    source_type: SourceType::Wikidata,
    name: "Multipaint image (QL mode 0)",
    extensions: &["bin"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x00, 0x00, 0x00, 0x42, 0x07, 0x20, 0x00, 0x20, 0x00, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
