use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105864361: FileFormat = FileFormat {
    id: 105_864_361,
    source_type: SourceType::Wikidata,
    name: "GraphiCode Programmable Device Format",
    extensions: &["pdf"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xAF, 0x01, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
