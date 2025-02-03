use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_72727499: FileFormat = FileFormat {
    id: 72_727_499,
    source_type: SourceType::Wikidata,
    name: "NVIDIA Scene",
    extensions: &["nvb"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4E, 0x56, 0x42, 0x21])],
            },
        }],
    }],
    related_formats: &[],
};
