use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105762725: FileType = FileType {
    file_format: &FileFormat {
        id: 105_762_725,
        source_type: SourceType::Wikidata,
        name: "Visual Studio Workflow service data",
        extensions: &[],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0xEF, 0xBB, 0xBF, 0x3C, 0x57, 0x6F, 0x72, 0x6B, 0x66, 0x6C, 0x6F, 0x77,
                        0x53, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x20, 0x6D, 0x63, 0x3A, 0x49,
                        0x67, 0x6E, 0x6F, 0x72, 0x61, 0x62, 0x6C, 0x65, 0x3D, 0x22, 0x73, 0x61,
                        0x70,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
