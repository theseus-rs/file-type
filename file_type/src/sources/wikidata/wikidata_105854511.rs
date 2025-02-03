use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105854511: FileFormat = FileFormat {
    id: 105_854_511,
    source_type: SourceType::Wikidata,
    name: "ZET compressed archive",
    extensions: &["zet"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4F, 0x5A, 0xDD, 0x12, 0x66, 0x0C, 0x00, 0x00, 0x00, 0x00, 0x64, 0x00, 0x01,
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x4F, 0x5A, 0xE0, 0x0C, 0x2B, 0x95, 0x00, 0x00,
                    0x00, 0x00, 0x00, 0x00, 0x4F, 0x5A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
