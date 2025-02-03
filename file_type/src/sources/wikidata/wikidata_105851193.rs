use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105851193: FileFormat = FileFormat {
    id: 105_851_193,
    source_type: SourceType::Wikidata,
    name: "TestGen data",
    extensions: &["tst"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x54, 0x47, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
