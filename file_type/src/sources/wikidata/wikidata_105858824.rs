use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105858824: FileFormat = FileFormat {
    id: 105_858_824,
    source_type: SourceType::Wikidata,
    name: "Project Dogwaffle layered bitmap",
    extensions: &["lyr"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4C, 0x59, 0x52, 0x32])],
            },
        }],
    }],
    related_formats: &[],
};
