use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105855741: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_741,
        source_type: SourceType::Wikidata,
        name: "CriticalSeeker Picture Packet",
        extensions: &["dsl"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x43, 0x72, 0x69, 0x74, 0x69, 0x63, 0x61, 0x6C, 0x53, 0x65, 0x65, 0x6B,
                        0x65, 0x72, 0x20, 0x50, 0x69, 0x63, 0x74, 0x75, 0x72, 0x65, 0x20, 0x50,
                        0x61, 0x63, 0x6B, 0x65, 0x74, 0x20, 0x46, 0x69, 0x6C, 0x65,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
