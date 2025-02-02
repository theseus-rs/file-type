use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105854025: FileFormat = FileFormat {
    id: 105_854_025,
    source_type: SourceType::Wikidata,
    name: "EZBIND archive",
    extensions: &["arc"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x45, 0x5A, 0x42, 0x49, 0x4E, 0x44])],
            },
        }],
    }],
    related_formats: &[],
};
