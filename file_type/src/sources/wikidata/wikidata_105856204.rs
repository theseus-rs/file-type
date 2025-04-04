use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856204: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_204,
        source_type: SourceType::Wikidata,
        name: "Microsoft Developer Studio Workspace",
        extensions: &["dsw"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4D, 0x69, 0x63, 0x72, 0x6F, 0x73, 0x6F, 0x66, 0x74, 0x20, 0x44, 0x65,
                        0x76, 0x65, 0x6C, 0x6F, 0x70, 0x65, 0x72, 0x20, 0x53, 0x74, 0x75, 0x64,
                        0x69, 0x6F, 0x20, 0x57, 0x6F, 0x72, 0x6B, 0x73, 0x70, 0x61, 0x63, 0x65,
                        0x20, 0x46, 0x69, 0x6C, 0x65, 0x2C, 0x20, 0x46, 0x6F, 0x72, 0x6D, 0x61,
                        0x74, 0x20, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
