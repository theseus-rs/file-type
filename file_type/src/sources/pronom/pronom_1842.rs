use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1842: FileFormat = FileFormat {
    id: 1_842,
    source_type: SourceType::Pronom,
    name: "Stata Data (DTA) Format",
    extensions: &["dta"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x3C, 0x73, 0x74, 0x61, 0x74, 0x61, 0x5F, 0x64, 0x74, 0x61, 0x3E, 0x3C, 0x68,
                    0x65, 0x61, 0x64, 0x65, 0x72, 0x3E, 0x3C, 0x72, 0x65, 0x6C, 0x65, 0x61, 0x73,
                    0x65, 0x3E, 0x31, 0x31, 0x38, 0x3C, 0x2F, 0x72, 0x65, 0x6C, 0x65, 0x61, 0x73,
                    0x65, 0x3E,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
