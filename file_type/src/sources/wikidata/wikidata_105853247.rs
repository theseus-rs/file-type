use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853247: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_247,
        source_type: SourceType::Wikidata,
        name: "Smart Install Maker project",
        extensions: &["smm"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x53, 0x6D, 0x61, 0x72, 0x74, 0x20, 0x49, 0x6E, 0x73, 0x74, 0x61, 0x6C,
                        0x6C, 0x20, 0x4D, 0x61, 0x6B, 0x65, 0x72, 0x20, 0x50, 0x72, 0x6F, 0x6A,
                        0x65, 0x63, 0x74, 0x20, 0x76, 0x2E,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
