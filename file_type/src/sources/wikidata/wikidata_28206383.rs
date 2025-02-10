use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_28206383: FileType = FileType {
    file_format: &FileFormat {
        id: 28_206_383,
        source_type: SourceType::Wikidata,
        name: "Image Processing Lab",
        extensions: &["ipl"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x69, 0x69, 0x69, 0x69, 0x04, 0x00, 0x00, 0x00, 0x31, 0x30, 0x30, 0x66,
                        0x64, 0x61, 0x74, 0x61,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
