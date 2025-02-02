use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_98923420: FileFormat = FileFormat {
    id: 98_923_420,
    source_type: SourceType::Wikidata,
    name: "slob",
    extensions: &["slob"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x21, 0x2D, 0x31, 0x53, 0x4C, 0x4F, 0x42, 0x1F,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
