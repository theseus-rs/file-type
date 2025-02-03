use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105857875: FileFormat = FileFormat {
    id: 105_857_875,
    source_type: SourceType::Wikidata,
    name: "Ensoniq SQ-80 EDM disk image",
    extensions: &["eds"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x0D, 0x0A, 0x53, 0x51, 0x2D, 0x38, 0x30, 0x20, 0x44, 0x69, 0x73, 0x6B,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
