use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_209054: FileType = FileType {
    file_format: &FileFormat {
        id: 209_054,
        source_type: SourceType::Wikidata,
        name: "Audio Video Interleave",
        extensions: &["avi"],
        media_types: &["video/vnd.avi"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x41, 0x56, 0x49, 0x20])],
                },
            }],
        }],
        related_formats: &[],
    },
};
