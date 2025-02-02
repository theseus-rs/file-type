use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105856889: FileFormat = FileFormat {
    id: 105_856_889,
    source_type: SourceType::Wikidata,
    name: "Game Description Language",
    extensions: &["gdl"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x28, 0x72, 0x6F, 0x6C, 0x65, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
