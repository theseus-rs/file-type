use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855580: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_580,
        source_type: SourceType::Wikidata,
        name: "olsrd configuration",
        extensions: &["olsr"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x23, 0x0D, 0x0A, 0x23, 0x20, 0x43, 0x6F, 0x6E, 0x66, 0x69, 0x67, 0x75,
                        0x72, 0x61, 0x74, 0x69, 0x6F, 0x6E, 0x20, 0x66, 0x69, 0x6C, 0x65, 0x20,
                        0x66, 0x6F, 0x72, 0x20, 0x6F, 0x6C, 0x73, 0x72, 0x2E, 0x6F, 0x72, 0x67,
                        0x20, 0x6F, 0x6C, 0x73, 0x72, 0x64,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
