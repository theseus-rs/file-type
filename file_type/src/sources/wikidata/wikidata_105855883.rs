use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855883: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_883,
        source_type: SourceType::Wikidata,
        name: "Concurrent DOS Serial Terminal Configuration",
        extensions: &["dat"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x43, 0x6F, 0x6E, 0x63, 0x75, 0x72, 0x72, 0x65, 0x6E, 0x74, 0x20, 0x44,
                        0x4F, 0x53, 0x20, 0x53, 0x65, 0x72, 0x69, 0x61, 0x6C, 0x20, 0x54, 0x65,
                        0x72, 0x6D, 0x69, 0x6E, 0x61, 0x6C, 0x20, 0x43, 0x6F, 0x6E, 0x66, 0x69,
                        0x67, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6F, 0x6E, 0x20, 0x66, 0x69, 0x6C,
                        0x65, 0x0D, 0x0A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
