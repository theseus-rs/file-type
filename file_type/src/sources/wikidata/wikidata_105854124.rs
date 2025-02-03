use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105854124: FileFormat = FileFormat {
    id: 105_854_124,
    source_type: SourceType::Wikidata,
    name: "iPer Advanced Embedded Hypertext",
    extensions: &["aeh"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x41, 0x4C, 0x44, 0x4F])],
            },
        }],
    }],
    related_formats: &[],
};
