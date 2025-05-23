use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856304: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_304,
        source_type: SourceType::Wikidata,
        name: "Visual Database Tools Query",
        extensions: &["dtq"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x5B, 0x30, 0x45, 0x32, 0x33, 0x32, 0x46, 0x46, 0x30, 0x2D, 0x42, 0x34,
                        0x36, 0x36, 0x2D, 0x31, 0x31, 0x63, 0x66, 0x2D, 0x41, 0x32, 0x34, 0x46,
                        0x2D, 0x30, 0x30, 0x41, 0x41, 0x30, 0x30, 0x41, 0x33, 0x45, 0x46, 0x46,
                        0x46, 0x2C, 0x20, 0x31, 0x2E, 0x30, 0x30, 0x5D, 0x0D, 0x0A, 0x42, 0x65,
                        0x67, 0x69, 0x6E, 0x20, 0x51, 0x75, 0x65, 0x72, 0x79, 0x20, 0x3D, 0x20,
                        0x22,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
