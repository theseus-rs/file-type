use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105855166: FileFormat = FileFormat {
    id: 105_855_166,
    source_type: SourceType::Wikidata,
    name: "FloorPlan Plus/3D Drawing",
    extensions: &["fp3"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x46, 0x6C, 0x6F, 0x6F, 0x72, 0x50, 0x6C, 0x61, 0x6E, 0x20, 0x50, 0x6C, 0x75,
                    0x73, 0x2F, 0x33, 0x44, 0x20, 0x44, 0x72, 0x61, 0x77, 0x69, 0x6E, 0x67, 0x20,
                    0x46, 0x69, 0x6C, 0x65,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
