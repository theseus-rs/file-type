use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105854614: FileFormat = FileFormat {
    id: 105_854_614,
    source_type: SourceType::Wikidata,
    name: "AvaaBook e-book",
    extensions: &["ava"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x41, 0x56, 0x41, 0x41, 0x42, 0x4F, 0x4F, 0x4B,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
