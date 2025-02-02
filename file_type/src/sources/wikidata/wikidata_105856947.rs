use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105856947: FileFormat = FileFormat {
    id: 105_856_947,
    source_type: SourceType::Wikidata,
    name: "Amigaguide hypertext document (var.2)",
    extensions: &["guide"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x40, 0x44, 0x41, 0x54, 0x41, 0x42, 0x41, 0x53, 0x45,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
