use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105855242: FileFormat = FileFormat {
    id: 105_855_242,
    source_type: SourceType::Wikidata,
    name: "AutoCAD Fast-load AutoLISP (FAS4)",
    extensions: &["fas"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x0D, 0x0A, 0x20, 0x46, 0x41, 0x53, 0x34, 0x2D, 0x46, 0x49, 0x4C, 0x45, 0x20,
                    0x3B, 0x20, 0x44, 0x6F, 0x20, 0x6E, 0x6F, 0x74, 0x20, 0x63, 0x68, 0x61, 0x6E,
                    0x67, 0x65, 0x20, 0x69, 0x74, 0x21, 0x0D, 0x0A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
