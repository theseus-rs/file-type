use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105853863: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_863,
        source_type: SourceType::Wikidata,
        name: "ANMA RED music data",
        extensions: &["amu"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x41, 0x4E, 0x4D, 0x41, 0x20, 0x6D, 0x75, 0x73, 0x69, 0x63, 0x20, 0x64,
                        0x61, 0x74, 0x61, 0x2E, 0x43, 0x6F, 0x64, 0x69, 0x6E, 0x67, 0x3A, 0x20,
                        0x41, 0x6E, 0x64, 0x72, 0x65, 0x20, 0x4C, 0x2E,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
