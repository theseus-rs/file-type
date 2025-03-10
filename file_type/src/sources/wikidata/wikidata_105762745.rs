use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105762745: FileType = FileType {
    file_format: &FileFormat {
        id: 105_762_745,
        source_type: SourceType::Wikidata,
        name: "Microsoft Dynamics AX Export (ASCII)",
        extensions: &[],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x45, 0x78, 0x70, 0x6F, 0x72, 0x74, 0x66, 0x69, 0x6C, 0x65, 0x20, 0x66,
                        0x6F, 0x72, 0x20, 0x41, 0x4F, 0x54, 0x20, 0x76, 0x65, 0x72, 0x73, 0x69,
                        0x6F, 0x6E, 0x20, 0x31, 0x2E, 0x30, 0x20, 0x6F, 0x72, 0x20, 0x6C, 0x61,
                        0x74, 0x65, 0x72, 0x0D, 0x0A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
