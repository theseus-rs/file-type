use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105858994: FileFormat = FileFormat {
    id: 105_858_994,
    source_type: SourceType::Wikidata,
    name: "App Inventor Blocks XML",
    extensions: &["bky"],
    media_types: &["text/xml"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x3C, 0x78, 0x6D, 0x6C])],
            },
        }],
    }],
    related_formats: &[],
};
