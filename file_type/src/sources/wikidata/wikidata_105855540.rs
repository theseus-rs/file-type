use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105855540: FileFormat = FileFormat {
    id: 105_855_540,
    source_type: SourceType::Wikidata,
    name: "3D Construction Kit Object",
    extensions: &["obj"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4F, 0x42, 0x4A, 0x54])],
            },
        }],
    }],
    related_formats: &[],
};
