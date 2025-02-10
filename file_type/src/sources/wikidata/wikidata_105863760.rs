use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105863760: FileType = FileType {
    file_format: &FileFormat {
        id: 105_863_760,
        source_type: SourceType::Wikidata,
        name: "Digital Tracker 8-channel module",
        extensions: &["mod"],
        media_types: &["audio/x-mod"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x46, 0x41, 0x30, 0x38])],
                },
            }],
        }],
        related_formats: &[],
    },
};
