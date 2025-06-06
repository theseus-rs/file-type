use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_76515119: FileType = FileType {
    file_format: &FileFormat {
        id: 76_515_119,
        source_type: SourceType::Wikidata,
        name: "Wings 3D mesh",
        extensions: &["wings"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x23, 0x21, 0x57, 0x49, 0x4E, 0x47, 0x53, 0x2D, 0x31, 0x2E, 0x30, 0x0D,
                        0x0A, 0x1A, 0x04, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
