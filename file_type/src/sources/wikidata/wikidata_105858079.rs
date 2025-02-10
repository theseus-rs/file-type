use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105858079: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_079,
        source_type: SourceType::Wikidata,
        name: "Godot Engine Import settings",
        extensions: &["import"],
        media_types: &["text/ini"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x5B, 0x72, 0x65, 0x6D, 0x61, 0x70, 0x5D])],
                },
            }],
        }],
        related_formats: &[],
    },
};
