use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105858904: FileFormat = FileFormat {
    id: 105_858_904,
    source_type: SourceType::Wikidata,
    name: "Compressed Champions' Interlace bitmap",
    extensions: &["cci"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x43, 0x49, 0x4E, 0x20, 0x31, 0x2E])],
            },
        }],
    }],
    related_formats: &[],
};
