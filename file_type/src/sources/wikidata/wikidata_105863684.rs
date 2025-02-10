use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105863684: FileType = FileType {
    file_format: &FileFormat {
        id: 105_863_684,
        source_type: SourceType::Wikidata,
        name: "AutoCAD compiled Menu",
        extensions: &["mnx"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x41, 0x75, 0x74, 0x6F, 0x43, 0x41, 0x44, 0x20, 0x43, 0x4F, 0x4D, 0x50,
                        0x49, 0x4C, 0x45, 0x44, 0x20, 0x6D, 0x65, 0x6E, 0x75, 0x20, 0x66, 0x69,
                        0x6C, 0x65,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
