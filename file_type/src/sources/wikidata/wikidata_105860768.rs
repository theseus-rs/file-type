use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105860768: FileFormat = FileFormat {
    id: 105_860_768,
    puid: "wikidata/105860768",
    name: "Godot Resource data",
    extensions: &["res"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x52, 0x53, 0x43, 0x43])],
            },
        }],
    }],
    related_formats: &[],
};
