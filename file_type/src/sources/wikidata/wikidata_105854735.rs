use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105854735: FileFormat = FileFormat {
    id: 105_854_735,
    source_type: SourceType::Wikidata,
    name: "Aksharamala Keymap Binary",
    extensions: &["akm"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x03, 0x31, 0x2E, 0x34])],
            },
        }],
    }],
    related_formats: &[],
};
