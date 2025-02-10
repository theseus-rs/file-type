use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105851831: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_831,
        source_type: SourceType::Wikidata,
        name: "Squirrel Module",
        extensions: &["sqm"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x53, 0x71, 0x75, 0x69, 0x72, 0x72, 0x65, 0x6C, 0x20, 0x4D, 0x6F, 0x64,
                        0x75, 0x6C, 0x65, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
