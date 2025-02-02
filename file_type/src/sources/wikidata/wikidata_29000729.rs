use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_29000729: FileFormat = FileFormat {
    id: 29_000_729,
    source_type: SourceType::Wikidata,
    name: "VMD",
    extensions: &["vmd"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x56, 0x4D, 0x4F, 0x44, 0x45, 0x4C, 0x20, 0x56, 0x45, 0x52, 0x53, 0x49, 0x4F,
                    0x4E, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
