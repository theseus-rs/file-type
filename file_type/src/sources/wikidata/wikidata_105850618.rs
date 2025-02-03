use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105850618: FileFormat = FileFormat {
    id: 105_850_618,
    source_type: SourceType::Wikidata,
    name: "Color Font Maker pattern (v1)",
    extensions: &["cfm"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x43, 0x46, 0x4D, 0x31])],
            },
        }],
    }],
    related_formats: &[],
};
