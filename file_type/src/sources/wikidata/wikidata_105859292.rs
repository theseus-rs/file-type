use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105859292: FileFormat = FileFormat {
    id: 105_859_292,
    source_type: SourceType::Wikidata,
    name: "Multipaint image (ZX)",
    extensions: &["bin"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x01, 0xFF, 0x00, 0x06, 0x0F, 0x20, 0x00, 0x18, 0x00, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
