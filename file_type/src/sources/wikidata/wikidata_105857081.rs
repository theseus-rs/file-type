use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857081: FileFormat = FileFormat {
    id: 105_857_081,
    source_type: SourceType::Wikidata,
    name: "Godot Engine project",
    extensions: &["godot"],
    media_types: &["text/ini"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x3B, 0x20, 0x45, 0x6E, 0x67, 0x69, 0x6E, 0x65, 0x20, 0x63, 0x6F, 0x6E, 0x66,
                    0x69, 0x67, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6F, 0x6E, 0x20, 0x66, 0x69, 0x6C,
                    0x65, 0x2E,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
