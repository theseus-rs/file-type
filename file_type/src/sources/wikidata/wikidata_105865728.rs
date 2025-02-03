use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105865728: FileFormat = FileFormat {
    id: 105_865_728,
    source_type: SourceType::Wikidata,
    name: "Password Tracker Deluxe Tracking List",
    extensions: &["pwt"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x50, 0x54, 0x43, 0x52, 0x52, 0x43])],
            },
        }],
    }],
    related_formats: &[],
};
