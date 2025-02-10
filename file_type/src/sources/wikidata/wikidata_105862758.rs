use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105862758: FileType = FileType {
    file_format: &FileFormat {
        id: 105_862_758,
        source_type: SourceType::Wikidata,
        name: "Windows DVD Maker project",
        extensions: &["msdvd"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x3C, 0x42, 0x75, 0x72, 0x6E, 0x57, 0x69, 0x7A, 0x61, 0x72, 0x64, 0x3E,
                        0x3C, 0x43, 0x6F, 0x6E, 0x74, 0x65, 0x6E, 0x74, 0x3E, 0x3C, 0x43, 0x6F,
                        0x6E, 0x74, 0x65, 0x6E, 0x74, 0x46, 0x69, 0x6C, 0x65, 0x20, 0x46, 0x69,
                        0x6C, 0x65, 0x6E, 0x61, 0x6D, 0x65, 0x3D, 0x22,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
