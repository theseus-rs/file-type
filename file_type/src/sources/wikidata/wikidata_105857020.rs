use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105857020: FileFormat = FileFormat {
    id: 105_857_020,
    source_type: SourceType::Wikidata,
    name: "GSP Family Tree",
    extensions: &["gft"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x0F, 0x00, 0x00, 0x01, 0x44])],
            },
        }],
    }],
    related_formats: &[],
};
