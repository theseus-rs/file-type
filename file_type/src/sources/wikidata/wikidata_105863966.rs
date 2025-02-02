use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105863966: FileFormat = FileFormat {
    id: 105_863_966,
    source_type: SourceType::Wikidata,
    name: "AMOD format module",
    extensions: &["amod"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x41, 0x4D, 0x4F, 0x44])],
            },
        }],
    }],
    related_formats: &[],
};
