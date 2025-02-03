use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105856157: FileFormat = FileFormat {
    id: 105_856_157,
    source_type: SourceType::Wikidata,
    name: "Twist DataBase",
    extensions: &["db"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xB1, 0x76, 0x23])],
            },
        }],
    }],
    related_formats: &[],
};
