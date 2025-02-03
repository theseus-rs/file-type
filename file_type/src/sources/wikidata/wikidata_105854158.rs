use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105854158: FileFormat = FileFormat {
    id: 105_854_158,
    source_type: SourceType::Wikidata,
    name: "Playmation Attributes",
    extensions: &["att"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x41, 0x54, 0x54, 0x52, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
