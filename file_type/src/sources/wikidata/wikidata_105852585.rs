use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852585: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_585,
        source_type: SourceType::Wikidata,
        name: "League Of Legends Skeleton",
        extensions: &["skl"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x72, 0x33, 0x64, 0x32, 0x73, 0x6B, 0x6C, 0x74,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
