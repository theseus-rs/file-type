use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105851051: FileFormat = FileFormat {
    id: 105_851_051,
    source_type: SourceType::Wikidata,
    name: "Traverse PC Desktop Survey Data",
    extensions: &["trv"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x23, 0x2C, 0x54, 0x52, 0x41, 0x56, 0x45, 0x52, 0x53, 0x45, 0x20, 0x50, 0x43,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
