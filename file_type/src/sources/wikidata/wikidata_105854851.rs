use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854851: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_851,
        source_type: SourceType::Wikidata,
        name: "AVM APEX sample studio sound bank",
        extensions: &["apex"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x41, 0x56, 0x4D, 0x20, 0x53, 0x61, 0x6D, 0x70, 0x6C, 0x65, 0x20, 0x53,
                        0x74, 0x75, 0x64, 0x69, 0x6F, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x4B, 0x75, 0x72, 0x7A,
                        0x77, 0x65, 0x69, 0x6C, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
