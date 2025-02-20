use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105862965: FileType = FileType {
    file_format: &FileFormat {
        id: 105_862_965,
        source_type: SourceType::Wikidata,
        name: "Microsoft Jet DB Workgroup Information",
        extensions: &["mdw"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x00, 0x01, 0x00, 0x00, 0x4A, 0x65, 0x74, 0x20, 0x53, 0x79, 0x73, 0x74,
                        0x65, 0x6D, 0x20, 0x44, 0x42, 0x20, 0x20, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
