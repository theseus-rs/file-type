use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852793: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_793,
        source_type: SourceType::Wikidata,
        name: "SpacEyes3D Viewer project",
        extensions: &["spv"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x53, 0x70, 0x61, 0x63, 0x45, 0x79, 0x65, 0x73, 0x33, 0x44, 0x20, 0x56,
                        0x69, 0x65, 0x77, 0x65, 0x72, 0x20, 0x50, 0x72, 0x6F, 0x6A, 0x65, 0x63,
                        0x74, 0x20, 0x46, 0x69, 0x6C, 0x65,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
