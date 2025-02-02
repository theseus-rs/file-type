use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105855607: FileFormat = FileFormat {
    id: 105_855_607,
    source_type: SourceType::Wikidata,
    name: "Oracle Help for Java mapping",
    extensions: &["oht"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x3C, 0x49, 0x44, 0x5F, 0x4D, 0x41, 0x50, 0x3E,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
