use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858468: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_468,
        source_type: SourceType::Wikidata,
        name: "Volleyball Playbook project",
        extensions: &["ebp"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x56, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E, 0x20, 0x20, 0x3D, 0x20, 0x56,
                        0x6F, 0x6C, 0x6C, 0x65, 0x79, 0x62, 0x61, 0x6C, 0x6C, 0x20, 0x50, 0x6C,
                        0x61, 0x79, 0x62, 0x6F, 0x6F, 0x6B, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
