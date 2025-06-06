use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852351: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_351,
        source_type: SourceType::Wikidata,
        name: "AutoCAD Compiled Shape (Bigfont) v1.0",
        extensions: &["shx"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x41, 0x75, 0x74, 0x6F, 0x43, 0x41, 0x44, 0x2D, 0x38, 0x36, 0x20, 0x62,
                        0x69, 0x67, 0x66, 0x6F, 0x6E, 0x74, 0x20, 0x31, 0x2E, 0x30, 0x0D, 0x0A,
                        0x1A, 0x08, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
