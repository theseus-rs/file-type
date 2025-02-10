use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105863086: FileType = FileType {
    file_format: &FileFormat {
        id: 105_863_086,
        source_type: SourceType::Wikidata,
        name: "Logistix Messages",
        extensions: &["msg"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4C, 0x4F, 0x47, 0x49, 0x53, 0x54, 0x49, 0x58, 0x20, 0x4D, 0x45, 0x53,
                        0x53, 0x41, 0x47, 0x45, 0x20, 0x46, 0x49, 0x4C, 0x45, 0x20, 0x20, 0x56,
                        0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E, 0x20, 0x31, 0x2E, 0x30, 0x0A, 0x0D,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
