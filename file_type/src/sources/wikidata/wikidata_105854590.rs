use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105854590: FileFormat = FileFormat {
    id: 105_854_590,
    source_type: SourceType::Wikidata,
    name: "SJGPlay Album info",
    extensions: &["alb"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x40, 0x41, 0x4C, 0x42, 0x55, 0x4D, 0x3A])],
            },
        }],
    }],
    related_formats: &[],
};
