use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858425: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_425,
        source_type: SourceType::Wikidata,
        name: "EazyDraw Library (binary)",
        extensions: &["ezdrawlibb"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x04, 0x0B, 0x73, 0x74, 0x72, 0x65, 0x61, 0x6D, 0x74, 0x79, 0x70, 0x65,
                        0x64, 0x81, 0xE8, 0x03, 0x84, 0x01, 0x40, 0x84, 0x84, 0x84, 0x13, 0x4E,
                        0x53, 0x4D, 0x75, 0x74, 0x61, 0x62, 0x6C, 0x65, 0x44, 0x69, 0x63, 0x74,
                        0x69, 0x6F, 0x6E, 0x61, 0x72, 0x79, 0x00, 0x84, 0x84, 0x0C, 0x4E, 0x53,
                        0x44, 0x69, 0x63, 0x74, 0x69, 0x6F, 0x6E, 0x61, 0x72, 0x79, 0x00, 0x84,
                        0x84, 0x08, 0x4E, 0x53, 0x4F, 0x62, 0x6A, 0x65, 0x63, 0x74,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
