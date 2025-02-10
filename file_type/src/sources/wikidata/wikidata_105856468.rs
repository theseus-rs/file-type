use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105856468: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_468,
        source_type: SourceType::Wikidata,
        name: "Visual Studio Work Item Query",
        extensions: &["wiq"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x3C, 0x57, 0x6F, 0x72, 0x6B, 0x49, 0x74, 0x65, 0x6D, 0x51, 0x75, 0x65,
                        0x72, 0x79, 0x20, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E, 0x3D, 0x22,
                        0x31, 0x22, 0x3E, 0x0D, 0x0A, 0x20, 0x20, 0x3C, 0x57, 0x69, 0x71, 0x6C,
                        0x3E,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
