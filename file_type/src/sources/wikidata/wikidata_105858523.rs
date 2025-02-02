use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105858523: FileFormat = FileFormat {
    id: 105_858_523,
    source_type: SourceType::Wikidata,
    name: "EZ-Art Professional bitmap",
    extensions: &["eza"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x45, 0x5A, 0x00, 0xC8])],
            },
        }],
    }],
    related_formats: &[],
};
