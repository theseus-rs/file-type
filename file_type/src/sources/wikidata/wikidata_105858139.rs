use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105858139: FileFormat = FileFormat {
    id: 105_858_139,
    source_type: SourceType::Wikidata,
    name: "PCE Raw Image disk image",
    extensions: &["pri"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x50, 0x52, 0x49, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
