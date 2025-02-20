use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858242: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_242,
        source_type: SourceType::Wikidata,
        name: "Electronics Workbench Circuit",
        extensions: &["ewb"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x45, 0x6C, 0x65, 0x63, 0x74, 0x72, 0x6F, 0x6E, 0x69, 0x63, 0x73, 0x20,
                        0x57, 0x6F, 0x72, 0x6B, 0x62, 0x65, 0x6E, 0x63, 0x68, 0x20, 0x43, 0x69,
                        0x72, 0x63, 0x75, 0x69, 0x74, 0x20, 0x46, 0x69, 0x6C, 0x65,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
