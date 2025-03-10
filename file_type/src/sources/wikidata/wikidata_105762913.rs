use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105762913: FileType = FileType {
    file_format: &FileFormat {
        id: 105_762_913,
        source_type: SourceType::Wikidata,
        name: "Termbase definition",
        extensions: &[],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x3C, 0x4F, 0x75, 0x74, 0x70, 0x75, 0x74, 0x3E, 0x3C, 0x4F, 0x62, 0x6A,
                        0x65, 0x63, 0x74, 0x3E, 0x3C, 0x54, 0x72, 0x65, 0x65, 0x3E, 0x3C, 0x4E,
                        0x6F, 0x64, 0x65, 0x20, 0x49, 0x44, 0x3D, 0x22, 0x30, 0x22, 0x3E, 0x3C,
                        0x54, 0x65, 0x78, 0x74, 0x43, 0x68, 0x69, 0x6C, 0x64,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
