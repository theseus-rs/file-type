use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105859513: FileFormat = FileFormat {
    id: 105_859_513,
    source_type: SourceType::Wikidata,
    name: "LZA animation/video",
    extensions: &["lza"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4C, 0x5A, 0x41, 0x4E, 0x49, 0x4D])],
            },
        }],
    }],
    related_formats: &[],
};
