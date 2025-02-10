use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105866068: FileType = FileType {
    file_format: &FileFormat {
        id: 105_866_068,
        source_type: SourceType::Wikidata,
        name: "PathMinder configuration (generic)",
        extensions: &["cnf", "pm4"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x50, 0x4D, 0x2E, 0x43, 0x4E, 0x46])],
                },
            }],
        }],
        related_formats: &[],
    },
};
