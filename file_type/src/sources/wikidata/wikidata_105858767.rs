use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105858767: FileFormat = FileFormat {
    id: 105_858_767,
    source_type: SourceType::Wikidata,
    name: "16 bit adaptive RLE compressed bitmap",
    extensions: &["jmg"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4A, 0x4D, 0x47, 0x20, 0x76])],
            },
        }],
    }],
    related_formats: &[],
};
