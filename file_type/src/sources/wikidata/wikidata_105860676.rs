use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105860676: FileFormat = FileFormat {
    id: 105_860_676,
    source_type: SourceType::Wikidata,
    name: "Reaction Data file",
    extensions: &["rd"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x24, 0x52, 0x44, 0x46, 0x49, 0x4C, 0x45, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
