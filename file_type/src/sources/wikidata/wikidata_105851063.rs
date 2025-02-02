use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105851063: FileFormat = FileFormat {
    id: 105_851_063,
    source_type: SourceType::Wikidata,
    name: "TVgenial Skin",
    extensions: &["tvgskin"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xB0, 0x54, 0x56, 0x67])],
            },
        }],
    }],
    related_formats: &[],
};
