use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105856542: FileFormat = FileFormat {
    id: 105_856_542,
    source_type: SourceType::Wikidata,
    name: "ACT! word processor document",
    extensions: &["wpa"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x57, 0x54, 0x43, 0x41])],
            },
        }],
    }],
    related_formats: &[],
};
