use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855217: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_217,
        source_type: SourceType::Wikidata,
        name: "Fxpansion Uninstall Script",
        extensions: &["fus"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x7B, 0x46, 0x78, 0x70, 0x61, 0x6E, 0x73, 0x69, 0x6F, 0x6E, 0x49, 0x6E,
                        0x73, 0x74, 0x61, 0x6C, 0x6C, 0x53, 0x79, 0x73, 0x74, 0x65, 0x6D, 0x7D,
                        0x0D, 0x0A, 0x7B, 0x55, 0x6E, 0x69, 0x6E, 0x73, 0x74, 0x61, 0x6C, 0x6C,
                        0x53, 0x63, 0x72, 0x69, 0x70, 0x74, 0x7D, 0x0D, 0x0A, 0x41, 0x50, 0x50,
                        0x4C, 0x49, 0x43, 0x41, 0x54, 0x49, 0x4F, 0x4E, 0x5F, 0x4E, 0x41, 0x4D,
                        0x45, 0x2C,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
