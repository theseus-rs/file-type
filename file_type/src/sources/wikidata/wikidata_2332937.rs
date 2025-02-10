use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_2332937: FileType = FileType {
    file_format: &FileFormat {
        id: 2_332_937,
        source_type: SourceType::Wikidata,
        name: "Windows Media Player Playlist",
        extensions: &["wpl"],
        media_types: &["application/vnd.ms-wpl"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4D, 0x69, 0x63, 0x72, 0x6F, 0x73, 0x6F, 0x66, 0x74, 0x20, 0x57, 0x69,
                        0x6E, 0x64, 0x6F, 0x77, 0x73, 0x20, 0x4D, 0x65, 0x64, 0x69, 0x61, 0x20,
                        0x50, 0x6C, 0x61, 0x79, 0x65, 0x72, 0x20, 0x2D, 0x2D, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
