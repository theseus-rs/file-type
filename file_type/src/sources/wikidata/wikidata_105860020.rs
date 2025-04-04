use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860020: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_020,
        source_type: SourceType::Wikidata,
        name: "MP4 Base Media v2 container video",
        extensions: &["mp4"],
        media_types: &["video/mp4"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x00, 0x00, 0x00, 0x20, 0x66, 0x74, 0x79, 0x70, 0x69, 0x73, 0x6F, 0x32,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
