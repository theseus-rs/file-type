use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105860345: FileFormat = FileFormat {
    id: 105_860_345,
    source_type: SourceType::Wikidata,
    name: "Rexx-Adventure Saved game",
    extensions: &["ras"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x54, 0x69, 0x74, 0x6C, 0x65, 0x3A])],
            },
        }],
    }],
    related_formats: &[],
};
