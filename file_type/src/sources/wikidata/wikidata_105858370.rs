use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105858370: FileFormat = FileFormat {
    id: 105_858_370,
    source_type: SourceType::Wikidata,
    name: "Godot Engine Exported Scene",
    extensions: &["escn"],
    media_types: &["text/ini"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x5B, 0x67, 0x64, 0x5F, 0x73, 0x63, 0x65, 0x6E, 0x65, 0x20, 0x6C, 0x6F, 0x61,
                    0x64, 0x5F, 0x73, 0x74, 0x65, 0x70, 0x73, 0x3D,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
