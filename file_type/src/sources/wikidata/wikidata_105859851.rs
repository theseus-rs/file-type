use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105859851: FileFormat = FileFormat {
    id: 105_859_851,
    source_type: SourceType::Wikidata,
    name: "VP3 sewing machine file",
    extensions: &["vp3"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x25, 0x76, 0x73, 0x6D, 0x25])],
            },
        }],
    }],
    related_formats: &[],
};
