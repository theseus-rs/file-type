use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105849990: FileFormat = FileFormat {
    id: 105_849_990,
    source_type: SourceType::Wikidata,
    name: "Common Loudspeaker Format binary (v1, Type 2)",
    extensions: &["cf2"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x41, 0xBD, 0x0A, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x76, 0x31, 0x2E,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
