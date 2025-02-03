use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105851968: FileFormat = FileFormat {
    id: 105_851_968,
    source_type: SourceType::Wikidata,
    name: "SAdT music composer module/song",
    extensions: &["sat"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x53, 0x41, 0x64, 0x54, 0x01])],
            },
        }],
    }],
    related_formats: &[],
};
