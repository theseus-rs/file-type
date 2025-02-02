use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105858520: FileFormat = FileFormat {
    id: 105_858_520,
    source_type: SourceType::Wikidata,
    name: "Vivaldi Music Dump format",
    extensions: &["bin"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4D, 0x44, 0x4D, 0x50])],
            },
        }],
    }],
    related_formats: &[],
};
