use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105862922: FileType = FileType {
    file_format: &FileFormat {
        id: 105_862_922,
        source_type: SourceType::Wikidata,
        name: "Paragon 5 Gameboy Tracker module",
        extensions: &["mgb"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x00, 0x00, 0x01, 0x00, 0x11, 0x00, 0x01, 0x00, 0x14, 0x00, 0x00, 0x00,
                        0x47, 0x61, 0x6D, 0x65, 0x42, 0x6F, 0x79, 0x20, 0x4D, 0x75, 0x73, 0x69,
                        0x63, 0x20, 0x4D, 0x6F, 0x64, 0x75, 0x6C, 0x65,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
