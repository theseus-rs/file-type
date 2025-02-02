use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105858616: FileFormat = FileFormat {
    id: 105_858_616,
    source_type: SourceType::Wikidata,
    name: "Advanced Image Coding bitmap",
    extensions: &["aic"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x41, 0x49, 0x43, 0x0A])],
            },
        }],
    }],
    related_formats: &[],
};
