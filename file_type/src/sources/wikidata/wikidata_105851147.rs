use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851147: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_147,
        source_type: SourceType::Wikidata,
        name: "Godot Engine Text Scene",
        extensions: &["tscn"],
        media_types: &["text/ini"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x5B, 0x67, 0x64, 0x5F, 0x73, 0x63, 0x65, 0x6E, 0x65, 0x20, 0x6C, 0x6F,
                        0x61, 0x64, 0x5F, 0x73, 0x74, 0x65, 0x70, 0x73, 0x3D,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
