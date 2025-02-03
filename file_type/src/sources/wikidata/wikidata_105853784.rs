use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105853784: FileFormat = FileFormat {
    id: 105_853_784,
    source_type: SourceType::Wikidata,
    name: "SKY compressed archive",
    extensions: &["sky"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xBC, 0x40, 0x10, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
