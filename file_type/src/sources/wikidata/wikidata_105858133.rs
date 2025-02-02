use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105858133: FileFormat = FileFormat {
    id: 105_858_133,
    source_type: SourceType::Wikidata,
    name: "ImgBurn Graph data",
    extensions: &["ibg"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x49, 0x42, 0x47, 0x44])],
            },
        }],
    }],
    related_formats: &[],
};
