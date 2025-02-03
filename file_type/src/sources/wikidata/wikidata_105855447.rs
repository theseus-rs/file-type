use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105855447: FileFormat = FileFormat {
    id: 105_855_447,
    source_type: SourceType::Wikidata,
    name: "FairUse Wizard project",
    extensions: &["fup"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x46, 0x55, 0x2D, 0x4D, 0x50, 0x41, 0x41, 0x21,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
